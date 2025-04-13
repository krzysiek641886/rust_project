/* IMPORTS FROM LIBRARIES */
use std::io::{self, Write};
use std::process::Command;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
/**
 * @brief Pings the Prusa Slicer executable.
 *
 * This function checks if the Prusa Slicer executable is reachable by running a command.
 *
 * @param prusa_path Path of Prusa Slicer
 * @return io::Result<()> Result indicating success or failure of the operation.
 */
pub fn ping_prusa_slicer(prusa_path: &str) -> io::Result<()> {
    let output = Command::new("prusa-slicer").arg("--help").output()?;
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
pub fn get_prusa_slicer_evaluation(
    _order: &SubmittedOrderData,
    _slicer_path: &str,
    _ws_path: &str,
) -> EvaluationResult {
    // prusa-slicer -g --load prusa_config.ini --output sliced.gcode '/Users/krzysztofmroz/Projects/rust_project/web_server_with_database/data_files/Main_plate_grey.stl'
    EvaluationResult { _price: 0.0 }
}

/* TESTS */
