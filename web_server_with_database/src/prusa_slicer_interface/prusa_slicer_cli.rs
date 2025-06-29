/* IMPORTS FROM LIBRARIES */
use std::io::{self, Write};
use std::process::Command;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use crate::prusa_slicer_interface::prusa_slicer_price_calculator::{
    calculate_the_price, EvaluatedPrintingParameters,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */
pub struct PrusaSlicerCli;

/* PRIVATE FUNCTIONS */
fn slice_the_stl_file(prusa_path: &str, file_name: &str, ws_path: &str) -> io::Result<String> {
    let prusa_config_path = format!("{}/data_files/prusa_config.ini", ws_path);
    let received_file_path = format!("{}/data_files/received_orders/{}", ws_path, file_name);
    let processed_file_path = format!(
        "{}/data_files/processed_orders/{}.gcode",
        ws_path, file_name
    );

    match Command::new(prusa_path)
        .arg("-g")
        .arg("--load")
        .arg(prusa_config_path)
        .arg("--output")
        .arg(&processed_file_path)
        .arg(received_file_path)
        .output()
    {
        Ok(_) => {
            return Ok(processed_file_path);
        }
        Err(e) => {
            eprintln!("Error running Prusa Slicer: {}", e);
            io::stderr()
                .write_all(format!("Error running Prusa Slicer: {}", e).as_bytes())
                .unwrap();
            return Err(e);
        }
    }
}

fn read_output_gcode_file(gcode_file_path: &str) -> EvaluatedPrintingParameters {
    let file = File::open(gcode_file_path).expect("Failed to open G-code file");
    let reader = BufReader::new(file);

    let mut estimated_time: u32 = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            if l.starts_with("; estimated printing time (normal mode)") {
                // Example line: "; estimated printing time (normal mode) = 1h 23m 45s"
                if let Some(eq_pos) = l.find('=') {
                    let time_str = l[eq_pos + 1..].trim();
                    let mut total_seconds: u32 = 0;
                    let mut current = time_str;
                    if let Some(h_pos) = current.find('h') {
                        let (hours, rest) = current.split_at(h_pos);
                        if let Ok(h) = hours.trim().parse::<u32>() {
                            total_seconds += h * 3600;
                        }
                        current = rest[1..].trim();
                    }
                    if let Some(m_pos) = current.find('m') {
                        let (mins, rest) = current.split_at(m_pos);
                        if let Ok(m) = mins.trim().parse::<u32>() {
                            total_seconds += m * 60;
                        }
                        current = rest[1..].trim();
                    }
                    if let Some(s_pos) = current.find('s') {
                        let (secs, _) = current.split_at(s_pos);
                        if let Ok(s) = secs.trim().parse::<u32>() {
                            total_seconds += s;
                        }
                    }
                    estimated_time = total_seconds;
                }
                break;
            }
        }
    }
    return EvaluatedPrintingParameters {
        time: estimated_time,
    };
}

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
        slicer_path: &str,
        ws_path: &str,
    ) -> EvaluationResult {
        let mut evaluation_result = EvaluationResult {
            name: order.name.clone(),
            email: order.email.clone(),
            copies_nbr: order.copies_nbr,
            file_name: order.file_name.clone(),
            price: 0.0,
        };

        let output_file_path = match slice_the_stl_file(slicer_path, &order.file_name, ws_path) {
            Ok(path) => path,
            Err(_) => {
                // You may want to handle the error differently or return a default EvaluationResult
                return evaluation_result;
            }
        };
        let evaluated_printing_parameters = read_output_gcode_file(output_file_path.as_str());
        evaluation_result.price =
            calculate_the_price(evaluated_printing_parameters, order.copies_nbr);
        return evaluation_result;
    }
}

/* TESTS */
