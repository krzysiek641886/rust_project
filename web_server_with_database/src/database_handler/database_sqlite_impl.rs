/* IMPORTS FROM LIBRARIES */
use rusqlite::Connection;
use std::io;
use std::sync::{Arc, Mutex};

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, StatusType};
use crate::database_handler::database_type_conversions::{
    chrono_to_datetime, datetime_to_chrono, str_to_print_material_type, str_to_print_type,
    str_to_status_type,
};

/* PRIVATE TYPES AND VARIABLES */
/* PUBLIC TYPES AND VARIABLES */
pub struct DatabaseSQLiteImpl {
    pub db_conn: Arc<Mutex<Option<Connection>>>,
}

/* PRIVATE FUNCTIONS */
fn read_orders_from_table(
    conn: &Connection,
    table_name: &str,
) -> io::Result<Vec<EvaluationResult>> {
    let query = format!("SELECT date, name, email, copies_nbr, file_name, price, material_type, print_type, status FROM {}", table_name);
    let mut stmt = conn.prepare(&query).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to prepare statement: {}", e),
        )
    })?;
    let order_iter = stmt
        .query_map([], |row| {
            let date_str: String = row.get(0)?;
            let date = datetime_to_chrono(&date_str);
            if date.is_err() {
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Wrong date format",
                    )),
                ));
            };
            let material_type_str: String = row.get(6)?;
            let material_type = str_to_print_material_type(&material_type_str);
            if material_type.is_err() {
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unknown material type",
                    )),
                ));
            };
            let print_type_str: String = row.get(7)?;
            let print_type = str_to_print_type(&print_type_str);
            if print_type.is_err() {
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    8,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unknown print type",
                    )),
                ));
            };
            let status_str: String = row.get(8)?;
            let status = str_to_status_type(&status_str);
            if status.is_err() {
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    7,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unknown status type",
                    )),
                ));
            };
            Ok(EvaluationResult {
                date: date.unwrap(),
                name: row.get(1)?,
                email: row.get(2)?,
                copies_nbr: row.get(3)?,
                file_name: row.get(4)?,
                price: row.get(5)?,
                material_type: material_type.unwrap(),
                print_type: print_type.unwrap(),
                status: status.unwrap(),
            })
        })
        .map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to query rows: {}", e))
        })?;

    let mut orders = Vec::new();
    for order in order_iter {
        orders.insert(
            0,
            order.map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to map row: {}", e))
            })?,
        );
    }
    Ok(orders)
}

fn write_evaluation_to_db(
    db_conn: &Connection,
    date: &str,
    name: &str,
    email: &str,
    copies_nbr: u32,
    file_name: &str,
    price: f64,
    material_type: &str,
    print_type: &str,
    status: &str,
) -> io::Result<()> {
    let params = rusqlite::params![
        date,
        name,
        email,
        copies_nbr,
        file_name,
        price,
        material_type,
        print_type,
        status
    ];
    let sql = "INSERT INTO Orders (date, name, email, copies_nbr, file_name, price, material_type, print_type, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";
    match db_conn.execute(sql, params) {
        Ok(_) => Ok(()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write to database",
        )),
    }
}

fn update_order_status_in_db(
    conn: &Connection,
    datetime: &str,
    new_status: &str,
) -> io::Result<()> {
    // Handle case where datetime ends with ' UTC'
    let datetime: &str = &datetime[0..19];

    let sql = "UPDATE Orders SET status = ? WHERE date = ?";
    let params = [new_status, datetime];
    match conn.execute(sql, params) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to update order status: {}", e),
        )),
    }
}

fn move_completed_orders_to_archive(db_conn: &Mutex<Option<Connection>>) {
    let guard = match db_conn.lock() {
        Ok(guard) => guard,
        Err(_) => {
            println!("Failed to obtain database lock");
            return;
        }
    };

    let conn = match guard.as_ref() {
        Some(conn) => conn,
        None => {
            println!("Database connection is not initialized");
            return;
        }
    };
    // Create archive table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS CompletedOrders (
            date datetime not null,
            name text not null,
            email text not null,
            copies_nbr integer not null,
            file_name text not null,
            price REAL not null,
            material_type text not null,
            print_type text not null,
            status text not null
        )",
        [],
    )
    .map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create archive table: {}", e),
        )
    })
    .unwrap();

    // Move completed orders to archive
    conn.execute(
        "INSERT INTO CompletedOrders SELECT * FROM Orders WHERE status = ?1 OR status = ?2",
        [
            StatusType::Canceled.to_string().as_str(),
            StatusType::Completed.to_string().as_str(),
        ],
    )
    .map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to archive completed orders: {}", e),
        )
    })
    .unwrap();

    // Now delete the archived orders from the original table
    conn.execute(
        "DELETE FROM Orders WHERE status = ?1 OR status = ?2",
        [
            StatusType::Completed.to_string().as_str(),
            StatusType::Canceled.to_string().as_str(),
        ],
    )
    .map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to archive completed orders: {}", e),
        )
    })
    .unwrap();
}

/* PUBLIC FUNCTIONS */
impl DatabaseInterfaceImpl for DatabaseSQLiteImpl {
    fn initialize_db(&self, db_name: &str) -> io::Result<()> {
        let conn = Connection::open(db_name).expect("Failed to open database");
        conn.execute(
            "create table if not exists Orders (
            date datetime not null,
            name text not null,
            email text not null,
            copies_nbr integer not null,
            file_name text not null,
            price REAL not null,
            material_type text not null,
            print_type text not null,
            status text not null
            )",
            [],
        )
        .expect("Failed to create Orders table");
        let mut db_conn = self.db_conn.lock().unwrap();
        *db_conn = Some(conn);
        return Ok(());
    }

    fn read_orders_from_db(&self) -> io::Result<Vec<EvaluationResult>> {
        let db_conn = self.db_conn.lock().unwrap();
        let conn = db_conn.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotConnected,
                "Database connection is not initialized",
            )
        })?;

        read_orders_from_table(conn, "Orders")
    }

    fn read_completed_orders_from_db(&self) -> io::Result<Vec<EvaluationResult>> {
        let db_conn = self.db_conn.lock().unwrap();
        let conn = db_conn.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotConnected,
                "Database connection is not initialized",
            )
        })?;

        read_orders_from_table(conn, "CompletedOrders")
    }

    fn add_evaluation_to_db(&self, eval_result: &EvaluationResult) -> io::Result<()> {
        let db_conn = self.db_conn.lock().unwrap();
        let conn = db_conn.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotConnected,
                "Database connection is not initialized",
            )
        })?;
        return write_evaluation_to_db(
            conn,
            chrono_to_datetime(&eval_result.date).as_str(),
            eval_result.name.as_str(),
            eval_result.email.as_str(),
            eval_result.copies_nbr,
            eval_result.file_name.as_str(),
            eval_result.price,
            eval_result.material_type.to_string().as_str(),
            eval_result.print_type.to_string().as_str(),
            StatusType::New.to_string().as_str(),
        );
    }

    fn modify_order_in_database(&self, datetime: &str, new_status: &str) -> io::Result<()> {
        let db_conn = self.db_conn.lock().unwrap();
        let conn = db_conn.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotConnected,
                "Database connection is not initialized",
            )
        })?;
        let update_result = update_order_status_in_db(conn, datetime, new_status);
        if update_result.is_ok() {
            // Clone the Arc<Mutex> to pass it to the new thread
            let db_conn_clone = self.db_conn.clone();
            // Spawn a new thread to move completed orders to archive
            std::thread::spawn(move || {
                move_completed_orders_to_archive(&db_conn_clone);
            });
        }
        return update_result;
    }
}
