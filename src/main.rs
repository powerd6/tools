use clap::{Parser, ValueEnum};
use log::{trace, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

use crate::{
    commands::{execute_command, Commands},
    file_system::real_file_system::RealFileSystem,
};

mod commands;
mod file_system;

mod module;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, global=true, value_enum, default_value_t=LogLevel::Info)]
    log_level: LogLevel,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum LogLevel {
    /// Shows pertinent information or errors to users
    Info,
    /// Includes non-fatal issues and warnings
    Warn,
    /// Useful to debug execution and inspect internal flow
    Debug,
    /// Extra verbosity, including contributor-centric traces and signals
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

fn main() {
    // Parse input
    let cli = Cli::parse();

    // Initialize loggers
    CombinedLogger::init(vec![TermLogger::new(
        cli.log_level.into(),
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
    trace!("Loggers initialized!");

    // Initialize FileSystem
    let file_system = RealFileSystem;
    trace!("File system initialized!");

    trace!("Executing command");
    execute_command(cli.command, file_system);

    trace!("Finished execution");
}
