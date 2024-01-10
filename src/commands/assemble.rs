use std::path::PathBuf;

use clap::{arg, Args};
use log::trace;

use crate::file_system::FileSystem;

use super::Command;

#[derive(Args)]
/// Assembles a directory into a powerd6 module
pub(crate) struct Assemble {
    /// The path of the directory to be processed
    #[arg(short, long, default_value = "./")]
    config: PathBuf,
}

impl<F: FileSystem> Command<F> for Assemble {
    fn execute(&self, _fs: &F) {
        trace!("Executing assemble");
        trace!("Finished assemble")
    }
}
