use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct ValidateArguments {
    #[arg(short, long, help = "The module file to validate")]
    module_path: PathBuf
}

pub(crate) fn validate_module(args: ValidateArguments) {
    todo!()
}