/* IMPORTS FROM LIBRARIES */
use std::io::{self, Write};
use std::process::Command;

/* IMPORTS FROM OTHER MODULES */

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
pub fn ping_orca_slicer(orca_path: &str) -> io::Result<()> {
    // Example command: ls the directory at orca_path
    let output = Command::new("ls").arg(orca_path).output()?;

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

/* TESTS */
