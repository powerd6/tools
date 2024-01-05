use std::path::{Path, PathBuf};

use log::trace;
use mockall::automock;

#[automock]
pub(crate) trait FileSystem {
    fn dir_exists(&self, path: &Path) -> bool;
    fn create_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
    fn create_dir_if_not_exist(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        /*
           TODO: Need unit tests.

           When first implementing using mockall to mock dir_exists and create_dir,
           there was a problem because you cannot "un-mock" a method of a mocked impl.

           When creating a custom struct for the purposes of this test, the signature
           doesn't allow for trivial tracking of which methods are called.
        */
        if self.dir_exists(path) {
            Ok(path.to_path_buf())
        } else {
            trace!("Creating directory {:?}", path);
            self.create_dir(path)
        }
    }

    fn file_exists(&self, path: &Path) -> bool;
    fn create_file(&self, path: &Path, contents: &str) -> Result<PathBuf, FileSystemError>;
    fn create_file_if_not_exists(
        &self,
        path: &Path,
        contents: &str,
    ) -> Result<PathBuf, FileSystemError> {
        /*
           TODO: Need unit tests.

           When first implementing using mockall to mock dir_exists and create_dir,
           there was a problem because you cannot "un-mock" a method of a mocked impl.

           When creating a custom struct for the purposes of this test, the signature
           doesn't allow for trivial tracking of which methods are called.
        */
        if self.file_exists(path) {
            Ok(path.to_path_buf())
        } else {
            trace!("Creating file {:?}", path);
            self.create_file(path, contents)
        }
    }

    fn get_dir_files(&self, path: &Path) -> Option<Vec<PathBuf>>;
}

#[derive(Debug)]
pub(crate) enum FileSystemError {
    UnableToCreateDirectory,
    UnableToCreateFile,
    UnableToWriteToFile,
}

pub(crate) mod real_file_system;
