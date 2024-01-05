use clap::Subcommand;

mod assemble;
mod initialize;

#[derive(Subcommand)]
pub(crate) enum Commands {
    Assemble(assemble::Assemble),
    Initialize(initialize::Initialize),
}

trait CommandExecutor {
    fn execute(&self);
}

pub(crate) fn execute_command(command: Commands) {
    match command {
        Commands::Assemble(a) => a.execute(),
        Commands::Initialize(a) => a.execute(),
    }
}