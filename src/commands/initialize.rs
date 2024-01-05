use std::path::PathBuf;

use clap::Args;
use log::{debug, trace};

use crate::file_system::FileSystem;

use super::Command;

#[derive(Args)]
/// Initializes a directory for a powerd6 module
pub(crate) struct Initialize {
    /// The path of the directory to be initialized
    #[arg(short, long, default_value = "./")]
    pub(crate) config: PathBuf,
}

impl<F: FileSystem> Command<F> for Initialize {
    fn execute(&self,fs: &F) {
        trace!("Executing initialize");
        // Create directory if it doesn't already exist
        let root = if fs.dir_exists(&self.config) {
            self.config.clone()
        } else {
            debug!("Creating root directory {:?}", &self.config);
            fs.create_dir(&self.config).unwrap()
        };
        debug!("Create module.yaml");
        debug!("Create authors directory");
        debug!("Create schema directory");
        debug!("Create contents directory");
    }
}
