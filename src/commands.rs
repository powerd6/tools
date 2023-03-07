use clap::Subcommand;

pub(crate) mod build;
pub(crate) mod validate;

#[derive(Subcommand)]
pub enum SubCommands {
    #[command(
        about = "Recursively build a directory into a module",
        arg_required_else_help = true,
    )]
    Build(build::BuildArguments),
    #[command(
        about = "Validates a module",
        arg_required_else_help = true,
    )]
    Validate(validate::ValidateArguments),
}
