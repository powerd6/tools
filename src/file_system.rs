use std::path::{Path, PathBuf};

use log::trace;

pub(crate) trait FileSystem {
    fn dir_exists(&self, path: &Path) -> bool;
    fn create_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
    fn create_dir_if_not_exists(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
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
        if self.file_exists(path) {
            Ok(path.to_path_buf())
        } else {
            trace!("Creating file {:?}", path);
            self.create_file(path, contents)
        }
    }

    fn get_dir_children(&self, path: &Path) -> Option<Vec<PathBuf>>;
}

#[derive(Debug)]
pub(crate) enum FileSystemError {
    UnableToCreateDirectory,
    UnableToCreateFile,
    UnableToWriteToFile,
}

pub(crate) mod real_file_system;

use mockall::mock;
/*
    Since mockall does not allow you to "un-mock" a method of a mocked impl,
    and the `FileSystem` trait has default implementations for some methods,
    this is required instead of an `#[automock]` property.

    Reference: https://github.com/asomers/mockall/issues/454
*/
mock! {
    FileSystem{}
    impl FileSystem for FileSystem{
        fn dir_exists(&self, path: &Path) -> bool;
        fn create_dir(&self, path: &Path) -> Result<PathBuf, FileSystemError>;
        fn file_exists(&self, path: &Path) -> bool;
        fn create_file(&self, path: &Path, contents: &str) -> Result<PathBuf, FileSystemError>;
        fn get_dir_children(&self, path: &Path) -> Option<Vec<PathBuf>>;
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::file_system::FileSystem;

    use super::MockFileSystem;

    #[test]
    fn it_creates_directory_if_missing() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(false);
        mock_fs
            .expect_create_dir()
            .return_once(|p| Ok(p.to_path_buf()));

        assert!(mock_fs.create_dir_if_not_exists(Path::new("./")).is_ok())
    }
    #[test]
    fn it_uses_existing_directory() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(true);
        mock_fs.expect_create_dir().never();

        assert!(mock_fs.create_dir_if_not_exists(Path::new("./")).is_ok())
    }

    #[test]
    fn it_creates_file_if_missing() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_file_exists().once().return_const(false);
        mock_fs
            .expect_create_file()
            .return_once(|p, _| Ok(p.to_path_buf()));

        assert!(mock_fs
            .create_file_if_not_exists(Path::new("./file"), "")
            .is_ok())
    }
    #[test]
    fn it_uses_existing_file() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_file_exists().once().return_const(true);
        mock_fs.expect_create_file().never();

        assert!(mock_fs
            .create_file_if_not_exists(Path::new("./file"), "")
            .is_ok())
    }
}
