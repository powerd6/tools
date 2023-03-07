use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct BuildArguments {
    #[arg(short, long, help = "The directory to start the building process from")]
    root_directory: PathBuf,
    #[arg(
        short,
        long,
        help = "The name of the output file",
        default_value = "module"
    )]
    output_file_name: String,
    #[arg(
        short,
        long,
        help = "Whether to execute the actions, or just describe them to verify what is about to be executed."
    )]
    dry_run: bool,
}

pub(crate) fn build_modules(args: BuildArguments) {
    todo!()
}
