use std::path::PathBuf;

use clap::{command, Args};

#[derive(Debug, Args)]
#[command(
    about = "Builds a directory into a powerd6 module.",
    long_about = "Builds a directory into a powerd6 module. This is a recursive process, so you should use the top-level directory of your project.",
    arg_required_else_help = true
)]
pub struct Command {
    #[arg(
        short = 'd',
        long = "dir",
        help = "The directory to start the building process on.",
        required = true
    )]
    root_directory: Vec<PathBuf>,
}
