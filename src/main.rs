use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use log::{LevelFilter, trace, error, debug};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, global=true, value_enum, default_value_t=LogLevel::Info)]
    log_level: LogLevel,

    #[command(subcommand, )]
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

impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Assembles a powerd6 module from a directory
    Assemble {
        /// The path of the directory to be assembled
        #[arg(short, long, value_name = "./")]
        config: Option<PathBuf>,
    },
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

    match &cli.command {
        Commands::Assemble { config } => {
            trace!("Executing assemble");
            debug!("Open config directory");
            debug!("Assemble module information");
            debug!("Assemble authorship information");
            debug!("Assemble schema information");
            debug!("Assemble content information");
        },
    }
}
