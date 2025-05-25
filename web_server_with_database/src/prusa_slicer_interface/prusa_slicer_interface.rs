/* IMPORTS FROM LIBRARIES */
use lazy_static::lazy_static;
use std::io;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use crate::prusa_slicer_interface::prusa_slicer_cli::PrusaSlicerCli;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    ws_path: Mutex<Option<String>>,
    slicer_exec_path: Mutex<Option<String>>,
    slicer_interface: Mutex<Box<dyn SlicerInterfaceImpl>>,
}

lazy_static! {
    static ref SLICER_IF_STATE: State = State {
        ws_path: Mutex::new(None),
        slicer_exec_path: Mutex::new(None),
        slicer_interface: Mutex::new(Box::new(PrusaSlicerCli {})),
    };
}

/* PRIVATE FUNCTIONS */
fn setup_paths_in_state(ws_path: &str, prusa_path: &str) -> io::Result<()> {
    let mut ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
    let mut slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
    *ws_path_lock = Some(ws_path.to_string());
    *slicer_exec_path_lock = Some(prusa_path.to_string());
    Ok(())
}

/* PUBLIC FUNCTIONS */
pub fn initialize_prusa_slicer_if(ws_path: &str, prusa_path: &str) -> io::Result<()> {
    if let Err(e) = setup_paths_in_state(ws_path, prusa_path) {
        return Err(io::Error::new(
            e.kind(),
            format!("Failed to set up paths in state: {}", e),
        ));
    }
    let slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();
    if let Err(e) = slicer_interface_lock.ping(prusa_path) {
        return Err(io::Error::new(
            e.kind(),
            format!("Failed to ping Prusa Slicer: {}", e),
        ));
    }
    Ok(())
}

pub fn get_prusa_slicer_evaluation(order: &SubmittedOrderData) -> EvaluationResult {
    let prusa_path = &*SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
    let workspace_path = &*SLICER_IF_STATE.ws_path.lock().unwrap();
    let slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();
    slicer_interface_lock.evaluate(
        order,
        prusa_path.as_deref().unwrap(),
        workspace_path.as_deref().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prusa_slicer_interface::prusa_slicer_mock::PrusaSlicerMock;

    /// Helper function to reset the global state
    fn reset_state_and_setup_mocked_interface(price_to_return: f64, ping_result: bool) {
        let mut ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
        let mut slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
        let mut slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();

        *ws_path_lock = None;
        *slicer_exec_path_lock = None;
        *slicer_interface_lock = Box::new(PrusaSlicerMock {
            price_to_return,
            ping_result,
        });
    }

    #[test]
    fn test_initialize_prusa_slicer_if_successfull_ping() {
        reset_state_and_setup_mocked_interface(42.0, true); // Reset state before the test

        let ws_path = "foobar";
        let prusa_path = "foobar";
        assert!(
            initialize_prusa_slicer_if(ws_path, prusa_path).is_ok(),
            "Failed to initialize Prusa Slicer interface"
        );

        let ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
        let slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
        assert_eq!(
            *ws_path_lock,
            Some(ws_path.to_string()),
            "Workspace path not set correctly"
        );
        assert_eq!(
            *slicer_exec_path_lock,
            Some(prusa_path.to_string()),
            "Slicer path not set correctly"
        );
    }

    #[test]
    fn test_initialize_prusa_slicer_if_failed_ping() {
        reset_state_and_setup_mocked_interface(42.0, false); // Reset state before the test

        let ws_path = "foobar";
        let prusa_path = "foobar";
        assert!(initialize_prusa_slicer_if(ws_path, prusa_path).is_err());
    }

    #[test]
    fn test_get_prusa_slicer_evaluation_success() {
        reset_state_and_setup_mocked_interface(42.0, true); // Reset state before the test

        let ws_path = "workspace_path";
        let prusa_path = "prusa_path";
        initialize_prusa_slicer_if(ws_path, prusa_path).unwrap();

        let order = SubmittedOrderData {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            copies_nbr: 5,
            file_name: "file.stl".to_string(),
            nbr_of_chunks: 42
        };

        let result = get_prusa_slicer_evaluation(&order);
        assert_eq!(result._price, 42.0, "Evaluation result price is incorrect");
    }
}
