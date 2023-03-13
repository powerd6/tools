use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use tools::{build_module, errors::ErrorCodes};

#[derive(Debug, Parser)]
#[command(bin_name = "powerd6", version, arg_required_else_help = true)]
#[command(about = "A set of tools to help working with powerd6 modules.", long_about = None,)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Builds a module from a directory
    #[command(arg_required_else_help = true)]
    Build {
        /// Root directory to start building from
        #[arg(required = true)]
        root_directory_path: PathBuf,
        /// The name of the resulting module file
        #[arg(default_value = "module")]
        output_file_name: String,
    },
}

pub(crate) fn run_cli() -> Result<(), ErrorCodes> {
    let cli = Cli::parse();

    println!("Here is what the CLI parsed: {:#?}", cli);

    match cli.command {
        Commands::Build {
            root_directory_path,
            output_file_name,
        } => build_module(root_directory_path, output_file_name),
    }
}
