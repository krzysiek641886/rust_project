/* IMPORTS FROM LIBRARIES */
use rusqlite::Connection;
use std::io;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, PrintMaterialType};

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
    material_type: &str,
) -> io::Result<()> {
    let params = rusqlite::params![name, email, copies_nbr, file_name, price, material_type];
    let sql = "INSERT INTO Orders (name, email, copies_nbr, file_name, price, material_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
    match db_conn.execute(sql, params) {
        Ok(_) => Ok(()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write to database",
        )),
    }
}

fn str_to_print_material_type(material: &str) -> Result<PrintMaterialType, &'static str> {
    match material {
        "PLA" => Ok(PrintMaterialType::PLA),
        "PET" => Ok(PrintMaterialType::PET),
        "ASA" => Ok(PrintMaterialType::ASA),
        _ => Err("Unknown material type"),
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
            price REAL not null,
            material_type text not null
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

        let mut stmt = conn
            .prepare("SELECT name, email, copies_nbr, file_name, price, material_type FROM Orders")
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to prepare statement: {}", e),
                )
            })?;
        let order_iter = stmt
            .query_map([], |row| {
                let material_type_str: String = row.get(5)?;
                let material_type = str_to_print_material_type(&material_type_str);
                if material_type.is_err() {
                    return Err(rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Unknown material type",
                        )),
                    ));
                };
                Ok(EvaluationResult {
                    name: row.get(0)?,
                    email: row.get(1)?,
                    copies_nbr: row.get(2)?,
                    file_name: row.get(3)?,
                    price: row.get(4)?,
                    material_type: material_type.unwrap(),
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
            eval_result.material_type.to_string().as_str(),
        );
    }
}
