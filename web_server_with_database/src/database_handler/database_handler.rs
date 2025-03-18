use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use std::sync::Mutex;
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

pub struct FormFields {
    pub name: Option<String>,
    pub email: Option<String>,
    pub copies_nbr: u32,
    pub file_name: Option<String>,
}

// Function to initialize the database connection
pub fn initialize_db(db_name: &str) {
    let mut db_name_lock = DB_HANDLER_STATE.db_name.lock().unwrap();
    *db_name_lock = db_name.to_string();
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
    let mut db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
    *db_conn = Some(conn);
}

pub fn add_form_submission_to_db(form_fields: FormFields) -> bool {
    let name = match form_fields.name {
        Some(name) => name,
        None => return false,
    };
    let email = match form_fields.email {
        Some(email) => email,
        None => return false,
    };
    if form_fields.copies_nbr < 1 {
        return false;
    }
    let file_name = match form_fields.file_name {
        Some(file_name) => file_name,
        None => return false,
    };
    write_submission_to_db(
        name.as_str(),
        email.as_str(),
        form_fields.copies_nbr.to_string().as_str(),
        file_name.as_str(),
    );

    return true;
}

fn write_submission_to_db(name: &str, email: &str, copes_nbr: &str, file_name: &str) -> bool {
    let db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
    let conn = db_conn
        .as_ref()
        .expect("Database connection is not initialized");

    conn.execute(
        "INSERT INTO Orders (name, email, copies_nbr, file_name) VALUES (?1, ?2, ?3, ?4)",
        &[&name, &email, copes_nbr, &file_name],
    )
    .is_ok()
}

pub fn read_orders_from_db() -> Result<Vec<FormFields>> {
    let db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
    let conn = db_conn
        .as_ref()
        .expect("Database connection is not initialized");

    let mut stmt = conn.prepare("SELECT name, email, copies_nbr, file_name FROM Orders")?;
    let order_iter = stmt.query_map([], |row| {
        Ok(FormFields {
            name: Some(row.get(0)?),
            email: Some(row.get(1)?),
            copies_nbr: row.get(2)?,
            file_name: Some(row.get(3)?),
        })
    })?;

    let mut orders = Vec::new();
    for order in order_iter {
        orders.push(order?);
    }

    Ok(orders)
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
        assert!(Path::new(db_name).exists(), "Database file was not created");

        // Check if the table was created
        {
            let db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
            let conn = db_conn
                .as_ref()
                .expect("Database connection is not initialized");
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='Orders'")
                .expect("Failed to prepare statement");
            let table_exists = stmt.exists([]).expect("Failed to execute query");
            assert!(table_exists, "Orders table was not created");
        }

        // Clean up the test database file
        if let Err(e) = fs::remove_file(db_name) {
            eprintln!("Failed to delete test database file: {}", e);
        }
    }

    #[test]
    fn test_add_form_submission_to_db() {
        let db_name = "test_db.sqlite";

        // Ensure the test database file does not exist before the test
        if Path::new(db_name).exists() {
            fs::remove_file(db_name).expect("Failed to delete existing test database file");
        }

        // Initialize the database
        initialize_db(db_name);

        // Create a FormFields instance
        let form_fields = FormFields {
            name: Some(String::from("John Doe")),
            email: Some(String::from("john.doe@example.com")),
            copies_nbr: 5,
            file_name: Some(String::from("file.txt")),
        };

        // Add form submission to the database
        let result = add_form_submission_to_db(form_fields);
        assert!(result, "Failed to add form submission to the database");

        // Clean up the test database file
        if let Err(e) = fs::remove_file(db_name) {
            eprintln!("Failed to delete test database file: {}", e);
        }
    }

    #[test]
    fn test_write_submission_to_db() {
        let db_name = "test_db.sqlite";

        // Ensure the test database file does not exist before the test
        if Path::new(db_name).exists() {
            fs::remove_file(db_name).expect("Failed to delete existing test database file");
        }

        // Initialize the database
        initialize_db(db_name);

        // Write submission to the database
        let result = write_submission_to_db("John Doe", "john.doe@example.com", "5", "file.txt");
        assert!(result, "Failed to write submission to the database");

        // Clean up the test database file
        if let Err(e) = fs::remove_file(db_name) {
            eprintln!("Failed to delete test database file: {}", e);
        }
    }
}
