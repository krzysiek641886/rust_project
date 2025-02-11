

// Function to initialize the database connection
pub fn hello_world_orca_slicer_interface() {
    println!("Hello, world!");
}

// pub fn initialize_orca_slicer_if(ws_path: &str, orca_path: &str) {
//     WS_PATH = Some(ws_path);
//     SLICER_EXEC_PATH = Some(orca_path);
//     println!("Orca Slicer Interface initialized with path: {}", SLICER_EXEC_PATH.unwrap());
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_orca_slicer_interface() {
        // Capture the output of the function
        let output = std::panic::catch_unwind(|| {
            hello_world_orca_slicer_interface();
        });

        // Check if the function executed without panicking
        assert!(output.is_ok());
    }
}
