/* IMPORTS FROM LIBRARIES */
use regex::Regex;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process::Command;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::SlicerInterfaceImpl;
use crate::common_utils::global_types::{EvaluatedPrintingParameters, SubmittedOrderData};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */
pub struct PrusaSlicerCli;

/* HELPER FUNCTIONS */
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

fn extract_time_from_line(line: &str, re: &Regex) -> Option<u32> {
    if let Some(caps) = re.captures(line) {
        let hours = caps
            .get(1)
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
        let minutes = caps
            .get(2)
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
        let seconds = caps
            .get(3)
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
        let estimated_time = hours * 3600 + minutes * 60 + seconds;
        return Some(estimated_time);
    }
    return None;
}

fn extract_material_from_line(line: &str, re: &Regex) -> Option<u32> {
    if let Some(caps) = re.captures(line) {
        if let Ok(material_mm) = caps.get(1).map_or(Ok(0), |m| m.as_str().parse::<u32>()) {
            return Some(material_mm);
        }
    }
    return None;
}

fn read_output_gcode_file(gcode_file_path: &str) -> EvaluatedPrintingParameters {
    let file: File = File::open(gcode_file_path).expect("Failed to open G-code file");
    let reader = BufReader::new(file);
    let re_time =
        Regex::new(r"^; estimated printing time \(normal mode\) = (?:(\d+)h )?(?:(\d+)m )?(\d+)s$")
            .unwrap();
    let re_material = Regex::new(r"^; filament used \[mm\] = (\d+)").unwrap();
    let mut time: Option<u32> = None;
    let mut material_mm: Option<u32> = None;
    for line in reader.lines() {
        let line_string = line.unwrap();
        let line_str = line_string.as_str();
        if re_time.is_match(line_str) {
            time = extract_time_from_line(line_str, &re_time);
        } else if re_material.is_match(line_str) {
            println!("Matched material regex in line: {}", line_str);
            material_mm = extract_material_from_line(line_str, &re_material);
        }
    }
    println!("Extracted time: {:?}, material_mm: {:?}", time, material_mm);
    if let (Some(t), Some(m)) = (time, material_mm) {
        return EvaluatedPrintingParameters {
            time: t,
            material_mm: m,
        };
    }
    panic!("Failed to find estimated printing time in G-code file");
}

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
    fn get_expected_print_parameters(
        &self,
        order: &SubmittedOrderData,
        slicer_path: &str,
        ws_path: &str,
    ) -> EvaluatedPrintingParameters {
        let output_file_path = match slice_the_stl_file(slicer_path, &order.file_name, ws_path) {
            Ok(path) => path,
            Err(_) => {
                // You may want to handle the error differently or return a default EvaluationResult
                return EvaluatedPrintingParameters {
                    time: 0,
                    material_mm: 0,
                };
            }
        };
        return read_output_gcode_file(output_file_path.as_str());
    }
}

/* TESTS */
