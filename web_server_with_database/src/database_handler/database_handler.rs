use rusqlite::Connection;
use std::sync::Mutex;
use lazy_static::lazy_static;
struct State {
    db_name: Mutex<String>,
    db_conn: Mutex<Option<Connection>>,
}

lazy_static! {
    static ref DB_HANDLER_STATE: State = State {
        db_name: Mutex::new(String::from("")),
        db_conn: Mutex::new(None),
    };
}

// Function to initialize the database connection
pub fn initialize_db(db_name: &str) {
    let mut db_name_lock = DB_HANDLER_STATE.db_name.lock().unwrap();
    *db_name_lock = db_name.to_string();
    let conn = Connection::open(db_name).expect("Failed to open database");
    conn.execute(
        "create table if not exists dummy_table (
            id integer primary key,
            name text not null unique)",
        [],
    )
    .expect("Failed to create dummy_table table");
    let mut db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
    *db_conn = Some(conn);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_initialize_db() {
        let db_name = "test_db.sqlite";

        // Ensure the test database file does not exist before the test
        if Path::new(db_name).exists() {
            fs::remove_file(db_name).expect("Failed to delete existing test database file");
        }

        // Call the function to initialize the database
        initialize_db(db_name);

        // Check if the database file was created
        assert!(
            Path::new(db_name).exists(),
            "Database file was not created"
        );

        // Check if the table was created
        {
            let db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
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
