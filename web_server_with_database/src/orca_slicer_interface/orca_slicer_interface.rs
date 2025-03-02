use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::orca_slicer_interface::orca_slicer_cli::ping_orca_slicer;

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

pub fn initialize_orca_slicer_if(ws_path: &str, orca_path: &str) {
    let mut ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
    let mut slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();

    *ws_path_lock = ws_path.to_string();
    *slicer_exec_path_lock = orca_path.to_string();

    if let Err(e) = ping_orca_slicer(orca_path) {
        panic!("Failed to ping Orca Slicer: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_orca_slicer_if() {
        initialize_orca_slicer_if("new_ws_path", "new_orca_path");

        let ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
        let slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();

        assert_eq!(*ws_path_lock, "new_ws_path");
        assert_eq!(*slicer_exec_path_lock, "new_orca_path");
    }
}
