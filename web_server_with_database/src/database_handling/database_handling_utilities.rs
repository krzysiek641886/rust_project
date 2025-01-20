use rusqlite::Connection;
use std::fs;
use std::sync::Mutex;

// Define a global mutable variable protected by a Mutex
pub static DB_CONN: Mutex<Option<Connection>> = Mutex::new(None);

// Function to initialize the database connection
pub fn initialize_db(db_name: &str) {
    let conn = Connection::open(db_name).expect("Failed to open database");
    conn.execute(
        "create table if not exists dummy_table (
            id integer primary key,
            name text not null unique)",
        [],
    )
    .expect("Failed to create dummy_table table");
    let mut db_conn = DB_CONN.lock().unwrap();
    *db_conn = Some(conn);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_initialize_db() {
        let db_name = "test_db.sqlite";

        // Ensure the test database file does not exist before the test
        if fs::metadata(db_name).is_ok() {
            fs::remove_file(db_name).expect("Failed to delete existing test database file");
        }

        // Call the function to initialize the database
        initialize_db(db_name);

        // Check if the database file was created
        assert!(
            fs::metadata(db_name).is_ok(),
            "Database file was not created"
        );

        // Check if the table was created
        {
            let db_conn = DB_CONN.lock().unwrap();
            let conn = db_conn
                .as_ref()
                .expect("Database connection is not initialized");
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='dummy_table'")
                .expect("Failed to prepare statement");
            let table_exists = stmt.exists([]).expect("Failed to execute query");
            assert!(table_exists, "dummy_table was not created");
        }

        // Clean up the test database file
        fs::remove_file(db_name).expect("Failed to delete test database file");
    }
}
