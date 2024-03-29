use clap::Subcommand;

use crate::file_system::FileSystem;

trait Command<F: FileSystem> {
    fn execute(&self, _: &F);
}

mod initialize;

#[derive(Subcommand)]
pub(crate) enum Commands {
    Initialize(initialize::Initialize),
}

pub(super) fn execute_command<F: FileSystem>(command: Commands, file_system: F) {
    match command {
        Commands::Initialize(a) => a.execute(&file_system),
    }
}
