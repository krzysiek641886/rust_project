/* IMPORTS FROM LIBRARIES */
use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};

/* PRIVATE TYPES AND VARIABLES */
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

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */
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

/* PUBLIC FUNCTIONS */

/**
 * @brief Initializes the database connection.
 *
 * This function sets up the database connection and creates the necessary tables
 * if they do not already exist.
 *
 * @param db_name Name of the database file.
 */
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

/**
 * @brief Adds a form submission to the database.
 *
 * This function validates the form submission data and writes it to the database.
 *
 * @param form_fields Submitted order data.
 * @return bool True if the submission was successfully added, false otherwise.
 */
pub fn add_form_submission_to_db(form_fields: SubmittedOrderData) -> bool {
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

/**
 * @brief Reads all orders from the database.
 *
 * This function retrieves all orders stored in the database and returns them
 * as a vector of `SubmittedOrderData` objects.
 *
 * @return Result<Vec<SubmittedOrderData>> A result containing a vector of orders
 *         if successful, or an error if the operation fails.
 */
pub fn read_orders_from_db() -> Result<Vec<SubmittedOrderData>> {
    let db_conn = DB_HANDLER_STATE.db_conn.lock().unwrap();
    let conn = db_conn
        .as_ref()
        .expect("Database connection is not initialized");

    let mut stmt = conn.prepare("SELECT name, email, copies_nbr, file_name FROM Orders")?;
    let order_iter = stmt.query_map([], |row| {
        Ok(SubmittedOrderData {
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

/**
 * @brief Retrieves a pending order from the database.
 *
 * This function returns a pending order if one exists. Currently, it is a placeholder.
 *
 * @return Option<SubmittedOrderData> A pending order or None if no orders are pending.
 */
pub fn get_pending_order() -> Option<SubmittedOrderData> {
    // Placeholder for the function
    return None;
}

/**
 * @brief Adds an evaluation result to the database.
 *
 * This function stores the evaluation result of an order in the database.
 * Currently, it is a placeholder.
 *
 * @param _slicer_evaluation_result Evaluation result to be added.
 */
pub fn add_evaluation_to_db(_slicer_evaluation_result: EvaluationResult) {
    // Placeholder for the function
}

/**
 * @brief Removes an order from the database.
 *
 * This function deletes an order from the database. Currently, it is a placeholder.
 *
 * @param _order Order to be removed.
 */
pub fn remove_order_from_db(_order: SubmittedOrderData) {
    // Placeholder for the function
}

/* TESTS */
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

        // Create a SubmittedOrderData instance
        let form_fields = SubmittedOrderData {
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
