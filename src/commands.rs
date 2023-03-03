use clap::Subcommand;

mod build;

#[derive(Subcommand)]
pub enum SubCommands {
    #[command(
        about = "Recursively build a directory into a module",
        arg_required_else_help = true,
    )]
    Build(build::BuildArguments),
    Validate,
}
