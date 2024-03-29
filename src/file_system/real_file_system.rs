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
            Err(_) => Err(FileSystemError::CreateDirectory),
        }
    }

    fn file_exists(&self, path: &Path) -> bool {
        metadata(path).map_or(false, |metadata: std::fs::Metadata| metadata.is_file())
    }

    fn create_file(&self, path: &Path, contents: &str) -> Result<PathBuf, FileSystemError> {
        match File::create(path) {
            Ok(mut f) => match f.write_all(contents.as_bytes()) {
                Ok(_) => Ok(PathBuf::from(path)),
                Err(_) => Err(FileSystemError::WriteToFile),
            },
            Err(_) => Err(FileSystemError::CreateFile),
        }
    }

    fn get_dir_children(&self, path: &Path) -> Option<Vec<PathBuf>> {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::file_system::FileSystem;

    use super::RealFileSystem;

    use testdir::testdir;

    #[test]
    fn it_returns_files_and_children() {
        let fs = RealFileSystem;

        let dir: PathBuf = testdir!();

        let file = fs.create_file(&dir.join("test.txt"), "").unwrap();
        let subdirectory = fs.create_dir(&dir.join("test")).unwrap();

        let mut actual = fs.get_dir_children(&dir).unwrap();
        let mut expected = vec![file, subdirectory];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected,)
    }
}
