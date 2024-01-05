use std::path::{PathBuf, Path};

use mockall::automock;

#[automock]
pub(crate) trait FileSystem {
    fn dir_exists(&self, path: &Path) -> bool;
    fn create_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
}

#[derive(Debug)]
pub(crate) enum FileSystemError {
    UnableToCreateDirectory,
}

pub(crate) mod real_file_system;
