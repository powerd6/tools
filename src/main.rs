use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    let _cli = Cli::parse();
}
