use log::warn;

/// VFS (Virtual File System) is an abstraction on top of the actual file system.
/// It serves as a layer of abstraction and allows for manipulation of files without touching the disk.

use crate::errors::ErrorCodes;

use std::path::PathBuf;


#[derive(Debug)]
pub(crate) struct VirtualFileSystem {
    pub(crate) types: Option<Vec<VirtualFileMapping>>,
    pub(crate) contents: Option<Vec<VirtualFileMapping>>,
}

#[derive(Debug)]
pub(crate) enum VirtualFileMapping {
    SingleFile {
        file_path: PathBuf,
    },
    Directory {
        root_file: PathBuf,
        extra_files: Vec<PathBuf>
    }
}

/// Traverses a directory recursively, creating a virtual representation of which files should be mapped into content and types
pub(crate) fn map_directory_to_module(root_directory_path: PathBuf) -> Result<VirtualFileSystem, ErrorCodes> {
    let types_subdirectory_path_fragment = PathBuf::from("./types");
    let types_directory: PathBuf = root_directory_path.join(types_subdirectory_path_fragment);

    Ok(VirtualFileSystem { types: map_types_directory(types_directory), contents: None })
}

fn map_types_directory(types_directory_path: PathBuf) -> Option<Vec<VirtualFileMapping>> {
    match (types_directory_path.exists(), types_directory_path.is_dir()) {
        (true, true) => todo!(),
        (true, false) => {
            warn!("Unable to process `types`. Expected a directory, but found a file instead.");
            None
        },
        (false, _) => {
            warn!("Did not find the `types` directory.");
            None
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;
    use testdir::testdir;

    #[test]
    fn mapping_types_fails_on_non_existing_directories() {
        testing_logger::setup();

        let empty_dir : PathBuf = testdir!();

        assert!(map_types_directory(empty_dir.join("something")).is_none());
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Did not find the `types` directory.");
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }

    #[test]
    fn mapping_types_fails_on_non_directories() {
        testing_logger::setup();

        let dir : PathBuf = testdir!();
        let file = dir.join("file.txt");
        std::fs::write(&file, "something").ok();

        assert!(map_types_directory(file).is_none());
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Unable to process `types`. Expected a directory, but found a file instead.");
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }
}