mod commands;

use clap::{command, Parser};
use commands::SubCommands;
use commands::build::build_modules;
use commands::validate::validate_module;

#[derive(Parser)]
#[command(
    bin_name = "pd6",
    version,
    about,
    long_about = None,
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}
pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Build(args) => build_modules(args),
        SubCommands::Validate(args) => validate_module(args),
    }
}
