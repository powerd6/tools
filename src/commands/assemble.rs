use std::path::PathBuf;

use clap::{arg, Args};
use log::trace;

use crate::file_system::FileSystem;

use super::Command;

#[derive(Args, Debug)]
/// Assembles a directory into a powerd6 module
pub(crate) struct Assemble {
    /// The path of the directory to be processed
    #[arg(short, long, default_value = "./")]
    config: PathBuf,
}

impl<F: FileSystem> Command<F> for Assemble {
    fn execute(&self, fs: &F) {
        trace!("Executing assemble: {:#?}", &self);
        let dir = &self.config;
        let children = fs.get_dir_children(&dir).unwrap();
        trace!("Processing {:?}", children);
        trace!("Process module information");
        trace!("Process author directory");
        trace!("Process content directory");
        trace!("Process schema directory");
        trace!("Finished assemble")
    }
}
