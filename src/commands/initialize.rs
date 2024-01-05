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
    config: PathBuf,
}

impl<F: FileSystem> Command<F> for Initialize {
    fn execute(&self, fs: &F) {
        trace!("Executing initialize");
        let root = self.initialize_root(fs);
        trace!("Initializing module.yaml");
        self.initialize_module_file(root, fs);
        debug!("Create authors directory");
        debug!("Create schema directory");
        debug!("Create contents directory");
    }
}
impl Initialize {
    fn initialize_root(&self, fs: &impl FileSystem) -> PathBuf {
        trace!("Initializing root directory");
        let root = if fs.dir_exists(&self.config) {
            self.config.clone()
        } else {
            debug!("Creating root directory {:?}", &self.config);
            fs.create_dir(&self.config).unwrap()
        };
        root
    }

    fn initialize_module_file(&self, root: PathBuf, fs: &impl FileSystem) {
        let module_file = &root.join("module.yaml");
        if !fs.file_exists(module_file) {
            debug!("Creating module.yaml");
            fs.create_file(module_file, "")
                .expect("Module file could not be created");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::file_system::MockFileSystem;

    use super::Initialize;

    #[test]
    fn it_creates_root_directory_when_needed() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(false);
        mock_fs
            .expect_create_dir()
            .once()
            .returning(|p| Ok(PathBuf::from(p)));

        Initialize {
            config: PathBuf::new(),
        }
        .initialize_root(&mock_fs);
    }
    #[test]
    fn it_uses_existing_root_directory() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(true);
        mock_fs.expect_create_dir().never();

        Initialize {
            config: PathBuf::new(),
        }
        .initialize_root(&mock_fs);
    }

    #[test]
    fn it_creates_module_file() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_file_exists().once().return_const(false);
        mock_fs
            .expect_create_file()
            .once()
            .returning(|p, _| Ok(PathBuf::from(p)));

        Initialize {
            config: PathBuf::new(),
        }
        .initialize_module_file(PathBuf::new(), &mock_fs);
    }

    #[test]
    fn it_uses_existing_module_file() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_file_exists().once().return_const(true);
        mock_fs.expect_create_file().never();

        Initialize {
            config: PathBuf::new(),
        }
        .initialize_module_file(PathBuf::new(), &mock_fs);
    }
}
