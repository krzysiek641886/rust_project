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
fn setup_paths_in_state(
    ws_path: &str,
    prusa_path: &str,
) -> io::Result<()> {
    let mut ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
    let mut slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
    *ws_path_lock = Some(ws_path.to_string());
    *slicer_exec_path_lock = Some(prusa_path.to_string());
    Ok(())
}

/* PUBLIC FUNCTIONS */
pub fn initialize_prusa_slicer_if(ws_path: &str, prusa_path: &str) {
    if let Err(e) = setup_paths_in_state(ws_path, prusa_path) {
        panic!("Failed to setup Prusa Slicer Interface: {:?}", e);
    }
    let slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();
    if let Err(e) = slicer_interface_lock.ping(prusa_path) {
        panic!("Failed to ping Prusa Slicer: {:?}", e);
    }
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

    #[test]
    fn test_initialize_prusa_slicer_if_successfull_ping() {
        // Verify that initialize_prusa_slicer_if doesn't panic
        // and that the state is set correctly
        {
            let mut slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();
            let mocked_interface = Box::new(PrusaSlicerMock {
                price_to_return: 42.0,
                ping_result: true,
            });
            *slicer_interface_lock = mocked_interface;
        }
        let ws_path = "foobar";
        let prusa_path = "foobar";
        initialize_prusa_slicer_if(ws_path, prusa_path);
        let ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
        let slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
        assert_eq!(*ws_path_lock, Some(ws_path.to_string()));
        assert_eq!(*slicer_exec_path_lock, Some(prusa_path.to_string()));
    }
    
    #[test]
    #[should_panic(expected = "Failed to ping Prusa Slicer")]
    fn test_initialize_prusa_slicer_if_failed_ping() {
        {
            let mut slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();
            let mocked_interface = Box::new(PrusaSlicerMock {
                price_to_return: 42.0,
                ping_result: false,
            });
            *slicer_interface_lock = mocked_interface;
        }
        let ws_path = "foobar";
        let prusa_path = "foobar";
        initialize_prusa_slicer_if(ws_path, prusa_path);
    }
}
