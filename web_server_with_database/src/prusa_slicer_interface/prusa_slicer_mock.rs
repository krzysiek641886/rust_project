/* IMPORTS FROM LIBRARIES */
use std::io;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */
pub struct PrusaSlicerMock
{
    pub price_to_return: f64,
    pub ping_result: bool,
}

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
impl SlicerInterfaceImpl for PrusaSlicerMock {
    fn ping(&self, _prusa_path: &str) -> io::Result<()> {
        if self.ping_result {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Error returned from mocked ping() function",
            ))
        }
    }

    fn evaluate(
        &self,
        _order: &SubmittedOrderData,
        _slicer_path: &str,
        _ws_path: &str,
    ) -> EvaluationResult {
        EvaluationResult { _price: self.price_to_return }
    }
}

/* TESTS */
