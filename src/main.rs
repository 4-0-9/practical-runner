use std::{
    error::Error,
    process::{self, Stdio},
};

#[allow(unused_imports)]
use clap::Parser;
use config::RunnerMenuColors;
use executables::get_executables;
use runner::Runner;

mod arguments;
mod config;
mod executables;
mod runner;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = arguments::Arguments::parse();

    let executables = get_executables()?;

    let mut runner = Runner::new(
        args.prompt,
        executables,
        RunnerMenuColors {
            font_color: args.font_color,
            font_color_active: args.font_color_active,
            background_color: args.background_color,
            background_color_active: args.background_color_active,
        },
    );

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
