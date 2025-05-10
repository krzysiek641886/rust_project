/* IMPORTS FROM LIBRARIES */
use rusqlite::Connection;
use std::io;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::SubmittedOrderData;

/* PRIVATE TYPES AND VARIABLES */
/* PUBLIC TYPES AND VARIABLES */
pub struct DatabaseSQLiteImpl {
    pub db_conn: Mutex<Option<Connection>>,
}

/* PRIVATE FUNCTIONS */
fn write_submission_to_db(db_conn: &Connection ,name: &str, email: &str, copes_nbr: &str, file_name: &str) -> bool {
    db_conn.execute(
        "INSERT INTO Orders (name, email, copies_nbr, file_name) VALUES (?1, ?2, ?3, ?4)",
        &[&name, &email, copes_nbr, &file_name],
    )
    .is_ok()
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
            file_name text not null)",
            [],
        )
        .expect("Failed to create Orders table");
        let mut db_conn = self.db_conn.lock().unwrap();
        *db_conn = Some(conn);
        return Ok(());
    }

    fn add_form_submission_to_db(&self, form_fields: SubmittedOrderData) -> io::Result<()> {
        let name = match form_fields.name {
            Some(name) => name,
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing name")),
        };
        let email = match form_fields.email {
            Some(email) => email,
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing email")),
        };
        if form_fields.copies_nbr < 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid number of copies"));
        }
        let file_name = match form_fields.file_name {
            Some(file_name) => file_name,
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing file name")),
        };
        let db_conn = self.db_conn.lock().unwrap();
        let conn = db_conn.as_ref().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotConnected, "Database connection is not initialized")
        })?;
        if !write_submission_to_db(
            conn,
            name.as_str(),
            email.as_str(),
            form_fields.copies_nbr.to_string().as_str(),
            file_name.as_str(),
        ) {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to write to database"));
        }
        Ok(())
    }

    fn read_orders_from_db(&self) -> io::Result<Vec<SubmittedOrderData>> {
        let db_conn = self.db_conn.lock().unwrap();
        let conn = db_conn
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Database connection is not initialized"))?;

        let mut stmt = conn
            .prepare("SELECT name, email, copies_nbr, file_name FROM Orders")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to prepare statement: {}", e)))?;
        let order_iter = stmt
            .query_map([], |row| {
                Ok(SubmittedOrderData {
                    name: Some(row.get(0)?),
                    email: Some(row.get(1)?),
                    copies_nbr: row.get(2)?,
                    file_name: Some(row.get(3)?),
                })
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to query rows: {}", e)))?;

        let mut orders = Vec::new();
        for order in order_iter {
            orders.push(order.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to map row: {}", e)))?);
        }
        Ok(orders)
    }

    fn get_pending_order(&self) -> Option<SubmittedOrderData> {
        match self.read_orders_from_db() {
            Ok(orders) => {
                if orders.is_empty() {
                    return None;
                }
                let order = orders[0].clone();
                return Some(order);
            }
            Err(_) => {
                println!("Error reading orders from database");
            }
        }
        return None;
    }
}
