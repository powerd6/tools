use std::path::PathBuf;

use clap::Args;
use log::{debug, trace};

use super::CommandExecutor;

#[derive(Args)]
/// Initializes a directory for a powerd6 module
pub(crate) struct Initialize {
    /// The path of the directory to be initialized
    #[arg(short, long, default_value = "./")]
    pub(crate) config: PathBuf,
}

impl CommandExecutor for Initialize {
    fn execute(&self) {
        trace!("Executing initialize");
        debug!(
            "Create config directory if not exists: {:?}",
            self.config
        );
        debug!("Create module.yaml");
        debug!("Create authors directory");
        debug!("Create schema directory");
        debug!("Create contents directory");
    }
}
