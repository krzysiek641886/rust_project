/* IMPORTS FROM LIBRARIES */
use std::io;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{
    EvaluatedPrintingParameters, PrintMaterialType, SubmittedOrderData,
};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */
pub struct PrusaSlicerMock {
    pub time: u32,
    pub material_mm: u32,
    pub ping_result: bool,
    pub material_type: PrintMaterialType,
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

    fn get_expected_print_parameters(
        &self,
        _order: &SubmittedOrderData,
        _slicer_path: &str,
        _ws_path: &str,
    ) -> EvaluatedPrintingParameters {
        EvaluatedPrintingParameters {
            time: self.time,
            material_mm: self.material_mm,
            material_type: self.material_type.clone(),
        }
    }
}

/* TESTS */
