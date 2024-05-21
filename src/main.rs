use std::{error::Error, process::{self, Stdio}};

use clap::Parser;
use executables::get_executables;

mod arguments;
mod executables;

fn main() -> Result<(), Box<dyn Error>> {
    let args = arguments::Arguments::parse();

    let executables = get_executables()?;

    Ok(())
}

fn run(program: impl ToString) {
    let _ = process::Command::new(program.to_string()).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn();
}
