/* IMPORTS FROM LIBRARIES */
use lazy_static::lazy_static;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use crate::prusa_slicer_interface::prusa_slicer_cli as prusa_cli;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    ws_path: Mutex<String>,
    slicer_exec_path: Mutex<String>,
}

lazy_static! {
    static ref SLICER_IF_STATE: State = State {
        ws_path: Mutex::new(String::from("")),
        slicer_exec_path: Mutex::new(String::from("")),
    };
}

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */

/**
 * @brief Initializes the Prusa Slicer interface.
 *
 * This function sets up the Prusa Slicer interface by updating the global state
 * with the workspace path and slicer executable path. It also pings the Prusa Slicer
 * to ensure it is reachable.
 *
 * @param ws_path Path to the workspace directory.
 * @param prusa_path Path to the Prusa Slicer executable.
 */
pub fn initialize_prusa_slicer_if(ws_path: &str, prusa_path: &str) {
    let mut ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
    let mut slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();

    *ws_path_lock = ws_path.to_string();
    *slicer_exec_path_lock = prusa_path.to_string();

    if let Err(e) = prusa_cli::ping_prusa_slicer(prusa_path) {
        panic!("Failed to ping Prusa Slicer: {:?}", e);
    }
}

/**
 * @brief Evaluates an order using the Prusa Slicer.
 *
 * This function takes a submitted order and returns an evaluation result.
 * Currently, it returns a placeholder result.
 *
 * @param _order Reference to the submitted order data.
 * @return EvaluationResult The result of the evaluation.
 */
pub fn get_prusa_slicer_evaluation(order: &SubmittedOrderData) -> EvaluationResult {
    let prusa_path = &*SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
    let workspace_path = &*SLICER_IF_STATE.ws_path.lock().unwrap();
    return prusa_cli::get_prusa_slicer_evaluation(order, prusa_path, workspace_path);
}

/* TESTS */
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_initialize_prusa_slicer_if() {
        //TBA
        // initialize_prusa_slicer_if("new_ws_path", "new_prusa_path");

        // let ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
        // let slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();

        // assert_eq!(*ws_path_lock, "new_ws_path");
        // assert_eq!(*slicer_exec_path_lock, "new_prusa_path");
    }
}
