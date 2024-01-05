use crate::file_system::FileSystemError;

use std::fs::create_dir_all;

use std::fs::metadata;

use std::path::Path;
use std::path::PathBuf;

use super::FileSystem;

pub(crate) struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn dir_exists(&self, path: &Path) -> bool {
        metadata(path).map_or(false, |metadata| metadata.is_dir())
    }

    fn create_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        match create_dir_all(path) {
            Ok(_) => Ok(PathBuf::from(path)),
            Err(_) => Err(FileSystemError::UnableToCreateDirectory),
        }
    }
}
