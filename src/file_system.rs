use std::path::PathBuf;

pub(crate) trait FileSystem {
    fn dir_exists(&self, path: &PathBuf) -> bool;
    fn create_dir(&self, path: &PathBuf) -> Result<PathBuf, FileSystemError>;
}

#[derive(Debug)]
pub(crate) enum FileSystemError {
    UnableToCreateDirectory,
}

pub(crate) mod real_file_system;
