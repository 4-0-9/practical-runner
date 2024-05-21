use std::{
    error::Error,
    process::{self, Stdio},
};

#[allow(unused_imports)]
use clap::Parser;
use executables::get_executables;
use runner::Runner;

mod arguments;
mod executables;
mod runner;
mod config;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // let args = arguments::Arguments::parse();

    let executables = get_executables()?;

    let mut runner = Runner::new(executables);

    if let Some(program) = runner.run() {
        run_program(program);
    }

    Ok(())
}

fn run_program(program: impl ToString) {
    let _ = process::Command::new(program.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}
