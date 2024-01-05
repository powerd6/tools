use std::error::Error;

use clap::Subcommand;

use crate::file_system::{FileSystem, self};

trait Command<F: FileSystem> {
    fn execute(&self, _: &F);
}

mod assemble;
mod initialize;

#[derive(Subcommand)]
pub(crate) enum Commands {
    Assemble(assemble::Assemble),
    Initialize(initialize::Initialize),
}

pub(super) fn execute_command<F: FileSystem>(command: Commands, file_system: F) {
    match command {
        Commands::Assemble(a) => a.execute(&file_system),
        Commands::Initialize(a) => a.execute(&file_system),
    }
}