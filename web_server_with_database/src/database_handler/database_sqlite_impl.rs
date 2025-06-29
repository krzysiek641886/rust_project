/* IMPORTS FROM LIBRARIES */
use rusqlite::Connection;
use std::io;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::EvaluationResult;

/* PRIVATE TYPES AND VARIABLES */
/* PUBLIC TYPES AND VARIABLES */
pub struct DatabaseSQLiteImpl {
    pub db_conn: Mutex<Option<Connection>>,
}

/* PRIVATE FUNCTIONS */
fn write_evaluation_to_db(
    db_conn: &Connection,
    name: &str,
    email: &str,
    copies_nbr: u32,
    file_name: &str,
    price: f64,
) -> io::Result<()> {
    let params = rusqlite::params![name, email, copies_nbr, file_name, price];
    let sql = "INSERT INTO Orders (name, email, copies_nbr, file_name, price) VALUES (?1, ?2, ?3, ?4, ?5)";
    match db_conn.execute(sql, params) {
        Ok(_) => Ok(()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write to database",
        )),
    }
}

/* PUBLIC FUNCTIONS */
impl DatabaseInterfaceImpl for DatabaseSQLiteImpl {
    fn initialize_db(&self, db_name: &str) -> io::Result<()> {
        let conn = Connection::open(db_name).expect("Failed to open database");
        conn.execute(
            "create table if not exists Orders (
            name text not null,
            email text not null,
            copies_nbr integer not null,
            file_name text not null,
            price REAL not null)",
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

        let mut stmt = conn
            .prepare("SELECT name, email, copies_nbr, file_name, price FROM Orders")
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to prepare statement: {}", e),
                )
            })?;
        let order_iter = stmt
            .query_map([], |row| {
                Ok(EvaluationResult {
                    name: row.get(0)?,
                    email: row.get(1)?,
                    copies_nbr: row.get(2)?,
                    file_name: row.get(3)?,
                    price: row.get(4)?,
                })
            })
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to query rows: {}", e))
            })?;

        let mut orders = Vec::new();
        for order in order_iter {
            orders.push(order.map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to map row: {}", e))
            })?);
        }
        Ok(orders)
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
            eval_result.name.as_str(),
            eval_result.email.as_str(),
            eval_result.copies_nbr,
            eval_result.file_name.as_str(),
            eval_result.price,
        );
    }
}
