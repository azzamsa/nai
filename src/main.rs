#![deny(unsafe_code)]
use std::process;

use clap::Parser;
use tracing_log::AsTrace;

use nai::{
    cli::Opts,
    config,
    exit_codes::ExitCode,
    output::{stderr, Printer},
    time::Time,
};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            stderr(&format!("Error: {:?}", err));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> miette::Result<ExitCode> {
    let opts = Opts::parse();

    // Tracing
    tracing_subscriber::fmt()
        .with_max_level(opts.verbose.log_level_filter().as_trace())
        .init();

    let config = config::load(&opts)?;
    tracing::debug!("Config: {:?}", config);

    let printer = Printer::new();
    for moment in config.moments {
        let time = Time::new(&moment.start_date)?;
        printer.print(&time.date()?, &time.duration()?.to_string(), &moment.format)?;
    }

    Ok(ExitCode::Success)
}
