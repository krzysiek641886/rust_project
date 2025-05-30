/* IMPORTS FROM LIBRARIES */
use std::io::{self, Write};
use std::process::Command;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */
pub struct PrusaSlicerCli;

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
impl SlicerInterfaceImpl for PrusaSlicerCli {
    /**
     * @brief Pings the Prusa Slicer executable.
     *
     * This function checks if the Prusa Slicer executable is reachable by running a command.
     *
     * @param prusa_path Path of Prusa Slicer
     * @return io::Result<()> Result indicating success or failure of the operation.
     */
    fn ping(&self, prusa_path: &str) -> io::Result<()> {
        let output = Command::new(prusa_path).arg("--help").output()?;
        if output.status.success() {
            Ok(())
        } else {
            io::stderr().write_all(&output.stderr)?;
            println!("Failed to ping Prusa Slicer at path: {:?}", prusa_path);
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to ping Prusa Slicer",
            ))
        }
    }

    /**
     * @brief Evaluates the Prusa Slicer through CLI.
     *
     * This function interacts with the Prusa Slicer executable via the command-line interface
     * to perform an evaluation or retrieve specific information.
     *
     * @param order Reference to the submitted order data.
     * @param slicer_exec_path Path to the Prusa Slicer executable.
     * @param ws_path Path to the workspace directory.
     * @return EvaluationResult Result containing the evaluation details.
     */
    fn evaluate(
        &self,
        order: &SubmittedOrderData,
        _slicer_path: &str,
        _ws_path: &str,
    ) -> EvaluationResult {
        // prusa-slicer -g --load prusa_config.ini --output sliced.gcode '/Users/username/Projects/rust_project/web_server_with_database/data_files/received_orders/stl_file.stl'

        EvaluationResult { 
            name: order.name.clone(),
            email: order.email.clone(),
            copies_nbr: order.copies_nbr,
            file_name: order.file_name.clone(),
            price: 42.0,
        }
    }
}

/* TESTS */
