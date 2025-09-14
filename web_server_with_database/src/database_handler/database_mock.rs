/* IMPORTS FROM LIBRARIES */
use std::io;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::DatabaseInterfaceImpl;
use crate::common_utils::global_types::EvaluationResult;

/* PRIVATE TYPES AND VARIABLES */
/* PUBLIC TYPES AND VARIABLES */
pub struct DatabaseMockImpl {}

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
impl DatabaseInterfaceImpl for DatabaseMockImpl {
    fn initialize_db(&self, _db_name: &str) -> io::Result<()> {
        return Ok(());
    }

    fn read_orders_from_db(&self) -> io::Result<Vec<EvaluationResult>> {
        let orders = Vec::new();
        return Ok(orders);
    }

    fn add_evaluation_to_db(&self, _eval_result: &EvaluationResult) -> io::Result<()> {
        Ok(())
    }

    fn modify_order_in_database(
        &self,
        _table_name: &str,
        _order_id: &str,
        _new_status: &str,
    ) -> io::Result<()> {
        Ok(())
    }

    fn read_completed_orders_from_db(&self) -> io::Result<Vec<EvaluationResult>> {
        let orders = Vec::new();
        return Ok(orders);
    }
}
