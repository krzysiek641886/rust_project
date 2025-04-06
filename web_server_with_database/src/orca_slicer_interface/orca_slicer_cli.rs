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
 * @brief Pings the Orca Slicer executable.
 *
 * This function checks if the Orca Slicer executable is reachable by running a command.
 *
 * @param orca_path Path to the Orca Slicer executable.
 * @return io::Result<()> Result indicating success or failure of the operation.
 */
pub fn ping_orca_slicer(orca_path: &str) -> io::Result<()> {
    let output = Command::new(orca_path).arg("--help").output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if output.status.success() {
        Ok(())
    } else {
        io::stderr().write_all(&output.stderr)?;
        println!("Failed to ping Orca Slicer at path: {:?}", orca_path);
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to ping Orca Slicer",
        ))
    }
}

/**
 * @brief Evaluates the Orca Slicer through CLI.
 *
 * This function interacts with the Orca Slicer executable via the command-line interface
 * to perform an evaluation or retrieve specific information.
 *
 * @param order Reference to the submitted order data.
 * @param slicer_exec_path Path to the Orca Slicer executable.
 * @param ws_path Path to the workspace directory.
 * @return EvaluationResult Result containing the evaluation details.
 */
pub fn get_orca_slicer_evaluation(
    _order: &SubmittedOrderData,
    _slicer_path: &str,
    _ws_path: &str,
) -> EvaluationResult {
    EvaluationResult { _price: 0.0 }
}

/* TESTS */
