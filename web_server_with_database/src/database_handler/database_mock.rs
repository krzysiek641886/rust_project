/* IMPORTS FROM LIBRARIES */
use std::io;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::SubmittedOrderData;

/* PRIVATE TYPES AND VARIABLES */
/* PUBLIC TYPES AND VARIABLES */
pub struct DatabaseMockImpl {}

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
impl DatabaseInterfaceImpl for DatabaseMockImpl {
    fn initialize_db(&self, _db_name: &str) -> io::Result<()> {
        return Ok(());
    }

    fn add_form_submission_to_db(&self, _form_fields: SubmittedOrderData) -> io::Result<()> {
        return Ok(());
    }

    fn read_orders_from_db(&self) -> io::Result<Vec<SubmittedOrderData>> {
        let orders = Vec::new();
        return Ok(orders);
    }
}
