use std::path::PathBuf;

use clap::Args;
use log::{debug, trace};

use crate::file_system::FileSystem;

use super::Command;

#[derive(Args)]
/// Assembles a powerd6 module from a directory
pub(crate) struct Assemble {
    /// The path of the directory to be assembled
    #[arg(short, long, default_value = "./")]
    pub(crate) config: PathBuf,
}

impl<F: FileSystem> Command<F> for Assemble {
    fn execute(&self, _: &F) {
        trace!("Executing assemble");
        debug!("Open config directory: {:?}", self.config);
        debug!("Assemble module information");
        debug!("Assemble authors information");
        debug!("Assemble schema information");
        debug!("Assemble contents information");
    }
}
