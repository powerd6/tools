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
    fn execute(&self, fs: &F) {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{commands::Command, file_system::MockFileSystem};

    use super::Initialize;

    #[test]
    fn test_it_creates_directory_when_needed() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(false);
        mock_fs
            .expect_create_dir()
            .once()
            .returning(|_| Ok(PathBuf::new()));

        Initialize {
            config: PathBuf::new(),
        }
        .execute(&mock_fs);
    }
    #[test]
    fn test_it_uses_existing_directory() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(true);
        mock_fs
            .expect_create_dir().never();

        Initialize {
            config: PathBuf::new(),
        }
        .execute(&mock_fs);
    }
}
