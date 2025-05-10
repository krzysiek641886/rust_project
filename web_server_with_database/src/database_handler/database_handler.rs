/* IMPORTS FROM LIBRARIES */
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::io::Result;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::database_handler::database_sqlite_impl::DatabaseSQLiteImpl;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    db_impl: Mutex<Box<dyn DatabaseInterfaceImpl>>,
}

lazy_static! {
    static ref DB_HANDLER_STATE: State = State {
        db_impl: Mutex::new(Box::new(DatabaseSQLiteImpl { db_conn: Mutex::new(None) })),
    };
}

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

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
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    database_handler_impl.initialize_db(db_name).expect("Failed to initialize database");
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
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    match database_handler_impl.add_form_submission_to_db(form_fields)
    {
        Ok(_) => return true,
        Err(_) => return false,
    }
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
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    return database_handler_impl.read_orders_from_db();
}

/**
 * @brief Retrieves a pending order from the database.
 *
 * This function returns a pending order if one exists. Currently, it is a placeholder.
 *
 * @return Option<SubmittedOrderData> A pending order or None if no orders are pending.
 */
pub fn get_pending_order() -> Option<SubmittedOrderData> {
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    return database_handler_impl.get_pending_order();
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
use crate::database_handler::database_mock::DatabaseMockImpl;

    /// Helper function to reset the global state
    fn reset_state_and_setup_mocked_interface() {
        let mut db_impl_lock = DB_HANDLER_STATE.db_impl.lock().unwrap();
        *db_impl_lock = Box::new(DatabaseMockImpl {});
    }

    #[test]
    fn test_initialize_db() {
        reset_state_and_setup_mocked_interface();
        initialize_db("test_db");
    }

    #[test]
    fn test_add_form_submission_to_db() {
        reset_state_and_setup_mocked_interface();
        let order = SubmittedOrderData {
            name: Some("John Doe".to_string()),
            email: Some("john.doe@example.com".to_string()),
            copies_nbr: 5,
            file_name: Some("file.stl".to_string()),
        };
        assert!(add_form_submission_to_db(order) == true);
    }

}
