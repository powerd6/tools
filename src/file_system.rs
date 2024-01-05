use std::path::{Path, PathBuf};

use mockall::automock;

#[automock]
pub(crate) trait FileSystem {
    fn dir_exists(&self, path: &Path) -> bool;
    fn create_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError>;

    fn file_exists(&self, path: &Path) -> bool;
    fn create_file(&self, path: &Path, contents: &str) -> Result<PathBuf, FileSystemError>;
}

#[derive(Debug)]
pub(crate) enum FileSystemError {
    UnableToCreateDirectory,
    UnableToCreateFile,
    UnableToWriteToFile,
}

pub(crate) mod real_file_system;
