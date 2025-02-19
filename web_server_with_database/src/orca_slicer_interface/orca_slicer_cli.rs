use std::process::Command;
use std::io::{self, Write};

pub fn ping_orca_slicer(orca_path: &str) -> io::Result<()> {
    // Example command: ls the directory at orca_path
    let output = Command::new("ls")
        .arg(orca_path)
        .output()?;

    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        println!("Pinging Orca Slicer at path: {:?}", orca_path);
    } else {
        io::stderr().write_all(&output.stderr)?;
        println!("Failed to ping Orca Slicer at path: {:?}", orca_path);
    }

    Ok(())
}