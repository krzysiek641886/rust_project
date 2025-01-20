use rusqlite::Connection;
use std::sync::Mutex;

// Define a global mutable variable protected by a Mutex
pub static DB_CONN: Mutex<Option<Connection>> = Mutex::new(None);

// Function to initialize the database connection
pub fn initialize_db(db_name: &str) {
    let conn = Connection::open(db_name).expect("Failed to open database");
    conn.execute(
        "create table if not exists dummy_table (
            id integer primary key,
            name text not null unique
            )",
        [],
    )
    .expect("Failed to create dummy_table table");
    let mut db_conn = DB_CONN.lock().unwrap();
    *db_conn = Some(conn);
}
