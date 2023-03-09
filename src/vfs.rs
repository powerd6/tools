use log::warn;
use walkdir::WalkDir;

/// VFS (Virtual File System) is an abstraction on top of the actual file system.
/// It serves as a layer of abstraction and allows for manipulation of files without touching the disk.
use crate::errors::ErrorCodes;

use std::{collections::BTreeSet, fs::read_dir, path::PathBuf};

#[derive(Debug)]
pub(crate) struct VirtualFileSystem {
    pub(crate) types: Option<BTreeSet<VirtualFileMapping>>,
    pub(crate) contents: Option<BTreeSet<VirtualFileMapping>>,
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub(crate) enum VirtualFileMapping {
    SingleFile {
        file_path: PathBuf,
    },
    Directory {
        root_file: PathBuf,
        extra_files: BTreeSet<PathBuf>,
    },
}

/// Traverses a directory recursively, creating a virtual representation of which files should be mapped into content and types
pub(crate) fn map_directory_to_module(
    root_directory_path: PathBuf,
) -> Result<VirtualFileSystem, ErrorCodes> {
    let types_subdirectory_path_fragment = PathBuf::from("./types");
    let types_directory: PathBuf = root_directory_path.join(types_subdirectory_path_fragment);

    Ok(VirtualFileSystem {
        types: map_types_directory(types_directory),
        contents: None,
    })
}

fn map_types_directory(types_directory_path: PathBuf) -> Option<BTreeSet<VirtualFileMapping>> {
    match (types_directory_path.exists(), types_directory_path.is_dir()) {
        (true, true) => {
            let mut results: BTreeSet<VirtualFileMapping> = BTreeSet::new();

            for entry in read_dir(types_directory_path)
                .into_iter()
                .flatten()
                .flat_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    results.insert(VirtualFileMapping::SingleFile { file_path: path });
                } else if path.is_dir() {
                    let underscore_file = path.join("_.json");
                    results.insert(VirtualFileMapping::Directory {
                        root_file: underscore_file,
                        extra_files: WalkDir::new(path)
                            .max_depth(2)
                            .into_iter()
                            .flatten()
                            .map(|e| e.into_path())
                            .filter(|p| p.is_file())
                            .filter(|f| f.file_stem().unwrap() != "_")
                            .collect(),
                    });
                }
            }
            Some(results)
        }
        (true, false) => {
            warn!("Unable to process `types`. Expected a directory, but found a file instead.");
            None
        }
        (false, _) => {
            warn!("Did not find the `types` directory.");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn mapping_types_fails_on_non_existing_directories() {
        testing_logger::setup();

        let empty_dir: PathBuf = testdir!();

        assert!(map_types_directory(empty_dir.join("something")).is_none());
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Did not find the `types` directory.");
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }

    #[test]
    fn mapping_types_fails_on_non_directories() {
        testing_logger::setup();

        let dir: PathBuf = testdir!();
        let file = dir.join("file.txt");
        std::fs::write(&file, "something").ok();

        assert!(map_types_directory(file).is_none());
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(
                captured_logs[0].body,
                "Unable to process `types`. Expected a directory, but found a file instead."
            );
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }

    #[test]
    fn mapping_types_reads_directory_correctly() {
        testing_logger::setup();

        let dir: PathBuf = testdir!();

        let single_type_path = dir.join("single_type.json");
        std::fs::write(&single_type_path, "").ok();

        let directory_type_dir = dir.join("directory_type");
        std::fs::create_dir(&directory_type_dir).ok();
        let nested_underscore_path = directory_type_dir.join("_.json");
        std::fs::write(&nested_underscore_path, "").ok();
        let nested_description_path = directory_type_dir.join("description.txt");
        std::fs::write(&nested_description_path, "").ok();

        let another_directory_type_dir = dir.join("another_directory_type");
        std::fs::create_dir(&another_directory_type_dir).ok();
        let another_nested_underscore_path = another_directory_type_dir.join("_.json");
        std::fs::write(&another_nested_underscore_path, "").ok();
        let nested_rendering_dir = another_directory_type_dir.join("rendering");
        std::fs::create_dir(&nested_rendering_dir).ok();
        let nested_template_path = nested_rendering_dir.join("txt.hjs");
        std::fs::write(&nested_template_path, "").ok();

        assert_eq!(
            map_types_directory(dir).unwrap(),
            BTreeSet::from([
                VirtualFileMapping::SingleFile {
                    file_path: single_type_path
                },
                VirtualFileMapping::Directory {
                    root_file: nested_underscore_path,
                    extra_files: BTreeSet::from([nested_description_path])
                },
                VirtualFileMapping::Directory {
                    root_file: another_nested_underscore_path,
                    extra_files: BTreeSet::from([nested_template_path])
                }
            ])
        )
    }
}
