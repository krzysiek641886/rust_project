/* IMPORTS FROM LIBRARIES */
use lazy_static::lazy_static;
use std::io::Result;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::EvaluationResult;
use crate::database_handler::database_sqlite_impl::DatabaseSQLiteImpl;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    db_impl: Mutex<Box<dyn DatabaseInterfaceImpl>>,
}

lazy_static! {
    static ref DB_HANDLER_STATE: State = State {
        db_impl: Mutex::new(Box::new(DatabaseSQLiteImpl {
            db_conn: Mutex::new(None)
        })),
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
    database_handler_impl
        .initialize_db(db_name)
        .expect("Failed to initialize database");
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
pub fn read_orders_from_db() -> Result<Vec<EvaluationResult>> {
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    return database_handler_impl.read_orders_from_db();
}

/**
 * @brief Adds an evaluation result to the database.
 *
 * This function stores the evaluation result of an order in the database.
 * Currently, it is a placeholder.
 *
 * @param _slicer_evaluation_result Evaluation result to be added.
 */
pub fn add_evaluation_to_db(slicer_evaluation_result: &EvaluationResult) -> Result<()> {
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    return database_handler_impl.add_evaluation_to_db(slicer_evaluation_result);
}

pub fn modify_order_in_database(datetime: &str, new_status: &str) -> Result<()> {
    let database_handler_impl = DB_HANDLER_STATE.db_impl.lock().unwrap();
    return database_handler_impl.modify_order_in_database(datetime, new_status);
}

/* TESTS */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common_utils::global_types::{PrintMaterialType, PrintType, StatusType};
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
    fn test_add_evaluation_to_db() {
        reset_state_and_setup_mocked_interface();
        let evaluation = EvaluationResult {
            date: chrono::Utc::now(),
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            copies_nbr: 1,
            file_name: "file.stl".to_string(),
            price: 100.0,
            material_type: PrintMaterialType::PLA,
            status: StatusType::New,
            print_type: PrintType::TBA,
        };
        let result = add_evaluation_to_db(&evaluation);
        assert!(result.is_ok());
        // No assertion as the mock does not return a value, just ensure no panic
    }
}
