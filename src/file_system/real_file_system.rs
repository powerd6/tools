use crate::file_system::FileSystemError;

use std::fs::create_dir_all;
use std::fs::File;

use std::fs::metadata;

use std::fs::read_dir;
use std::io::Write;
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

    fn file_exists(&self, path: &Path) -> bool {
        metadata(path).map_or(false, |metadata: std::fs::Metadata| metadata.is_file())
    }

    fn create_file(&self, path: &Path, contents: &str) -> Result<PathBuf, FileSystemError> {
        match File::create(path) {
            Ok(mut f) => match f.write_all(contents.as_bytes()) {
                Ok(_) => Ok(PathBuf::from(path)),
                Err(_) => Err(FileSystemError::UnableToWriteToFile),
            },
            Err(_) => Err(FileSystemError::UnableToCreateFile),
        }
    }

    fn get_dir_files(&self, path: &Path) -> Option<Vec<PathBuf>> {
        match read_dir(path) {
            Ok(entries) => {
                let files: Vec<PathBuf> = entries
                    .filter_map(|entry| entry.ok().map(|e| e.path()))
                    .collect();

                if files.is_empty() {
                    None
                } else {
                    Some(files)
                }
            }
            Err(_) => None,
        }
    }
}
