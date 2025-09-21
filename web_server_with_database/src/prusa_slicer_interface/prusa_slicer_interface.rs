/* IMPORTS FROM LIBRARIES */
use lazy_static::lazy_static;
use std::io;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{
    EvaluationResult, PrinterConfiguration, StatusType, SubmittedOrderData,
};
use crate::prusa_slicer_interface::prusa_slicer_cli::PrusaSlicerCli;
use crate::prusa_slicer_interface::prusa_slicer_price_calculator::calculate_the_price;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    ws_path: Mutex<Option<String>>,
    slicer_exec_path: Mutex<Option<String>>,
    slicer_interface: Mutex<Box<dyn SlicerInterfaceImpl>>,
    printer_configuration: Mutex<PrinterConfiguration>,
}

lazy_static! {
    static ref SLICER_IF_STATE: State = State {
        ws_path: Mutex::new(None),
        slicer_exec_path: Mutex::new(None),
        slicer_interface: Mutex::new(Box::new(PrusaSlicerCli {})),
        printer_configuration: Mutex::new(PrinterConfiguration {
            material_rate_pla: 0,
            material_rate_pet: 0,
            material_rate_asa: 0,
            hourly_rate_time_threshold: [0, 10, 100],
            hourly_rate_pla_price: [30, 25, 20],
            hourly_rate_pet_price: [35, 30, 25],
            hourly_rate_asa_price: [40, 35, 30],
        }),
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

fn open_and_parse_json_file_into_printer_configuration(
    file_path: &str,
) -> io::Result<PrinterConfiguration> {
    let file_content = std::fs::read_to_string(file_path)?;
    let printer_config: PrinterConfiguration = serde_json::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(printer_config)
}

fn set_printer_configuration(ws_path: &str, printer_configuration: &str) -> io::Result<()> {
    let full_path = format!("{}/{}", ws_path, printer_configuration);
    let config = open_and_parse_json_file_into_printer_configuration(&full_path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to parse printer configuration: {}", e),
        )
    })?;
    let mut printer_config_lock = SLICER_IF_STATE.printer_configuration.lock().unwrap();
    *printer_config_lock = config;
    Ok(())
}

/* PUBLIC FUNCTIONS */
pub fn initialize_prusa_slicer_if(
    ws_path: &str,
    prusa_path: &str,
    price_params_rel_path: &str,
) -> io::Result<()> {
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
    if let Err(e) = set_printer_configuration(ws_path, price_params_rel_path) {
        return Err(io::Error::new(
            e.kind(),
            format!("Failed to set printer configuration: {}", e),
        ));
    }
    Ok(())
}

pub fn get_prusa_slicer_evaluation(order: &SubmittedOrderData) -> EvaluationResult {
    let current_utc_time = chrono::Utc::now();
    let prusa_path = &*SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
    let workspace_path = &*SLICER_IF_STATE.ws_path.lock().unwrap();
    let slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();
    let print_params = slicer_interface_lock.get_expected_print_parameters(
        order,
        prusa_path.as_deref().unwrap(),
        workspace_path.as_deref().unwrap(),
    );
    let printer_configuration = SLICER_IF_STATE.printer_configuration.lock().unwrap();
    let price = calculate_the_price(&printer_configuration, print_params, order.copies_nbr);

    EvaluationResult {
        date: current_utc_time,
        name: order.name.clone(),
        email: order.email.clone(),
        copies_nbr: order.copies_nbr,
        file_name: order.file_name.clone(),
        price,
        material_type: order.material_type.clone(),
        print_type: order.print_type.clone(),
        status: StatusType::New,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common_utils::global_types::{PrintMaterialType, PrintType},
        prusa_slicer_interface::prusa_slicer_mock::PrusaSlicerMock,
    };

    /// Helper function to reset the global state and set paths
    fn reset_state_and_setup_mocked_interface(
        ping_result: bool,
        time_result: u32,
        material_mm_result: u32,
        ws_path: Option<&str>,
        prusa_path: Option<&str>,
    ) {
        let mut ws_path_lock = SLICER_IF_STATE.ws_path.lock().unwrap();
        let mut slicer_exec_path_lock = SLICER_IF_STATE.slicer_exec_path.lock().unwrap();
        let mut slicer_interface_lock = SLICER_IF_STATE.slicer_interface.lock().unwrap();

        *ws_path_lock = ws_path.map(|s| s.to_string());
        *slicer_exec_path_lock = prusa_path.map(|s| s.to_string());
        *slicer_interface_lock = Box::new(PrusaSlicerMock {
            time: time_result,
            material_mm: material_mm_result,
            ping_result,
            material_type: PrintMaterialType::PLA, // Assuming PLA for simplicity
        });
    }

    #[test]
    fn test_initialize_prusa_slicer_if_successfull_ping() {
        // Set up mock with ping_result = true
        reset_state_and_setup_mocked_interface(true, 1234, 5678, None, None);

        let ws_path = "foobar";
        let prusa_path = "foobar";
        // This will call ping() on the mock, which returns Ok(())
        assert!(
            initialize_prusa_slicer_if(ws_path, prusa_path, "foo").is_ok(),
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
        // Set up mock with ping_result = false
        reset_state_and_setup_mocked_interface(false, 1234, 5678, None, None);

        let ws_path = "foobar";
        let prusa_path = "foobar";
        // This will call ping() on the mock, which returns Err
        assert!(initialize_prusa_slicer_if(ws_path, prusa_path, "foo").is_err());
    }

    #[test]
    fn test_get_prusa_slicer_evaluation_success() {
        // Set up mock and state directly
        let ws_path = "workspace_path";
        let prusa_path = "prusa_path";
        reset_state_and_setup_mocked_interface(true, 1234, 5678, Some(ws_path), Some(prusa_path));

        let order = SubmittedOrderData {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            copies_nbr: 5,
            file_name: "file.stl".to_string(),
            nbr_of_chunks: 42,
            material_type: PrintMaterialType::PLA,
            print_type: PrintType::Standard,
        };

        let result = get_prusa_slicer_evaluation(&order);
        assert!(result.price > 0.0, "Evaluation result price is incorrect");
    }
}
