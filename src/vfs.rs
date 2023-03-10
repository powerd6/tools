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

const TYPES_DIRECTORY: &str = "types";
const CONTENTS_DIRECTORY: &str = "contents";
const UNDERSCORE_FILE_NAME: &str = "_.json";
const RENDERING_DIRECTORY: &str = "rendering";

/// Traverses a directory recursively, creating a virtual representation of which files should be mapped into content and types
pub(crate) fn map_directory_to_module(
    root_directory_path: PathBuf,
) -> Result<VirtualFileSystem, ErrorCodes> {
    let types_directory: PathBuf = root_directory_path.join(TYPES_DIRECTORY);
    let contents_directory: PathBuf = root_directory_path.join(CONTENTS_DIRECTORY);

    Ok(VirtualFileSystem {
        types: map_types_directory(types_directory),
        contents: map_contents_directory(contents_directory),
    })
}

fn map_contents_directory(contents_directory: PathBuf) -> Option<BTreeSet<VirtualFileMapping>> {
    match (contents_directory.exists(), contents_directory.is_dir()) {
        (true, true) => todo!(),
        (true, false) => {
            warn!(
                "Unable to process `{}`. Expected a directory, but found a file instead.",
                CONTENTS_DIRECTORY
            );
            None
        }
        (false, _) => {
            warn!("Did not find the `{}` directory.", CONTENTS_DIRECTORY);
            None
        }
    }
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
                    let underscore_file = path.join(UNDERSCORE_FILE_NAME);
                    match underscore_file.exists() {
                        true => {
                            let sibling_files = read_dir(path.clone())
                                .into_iter()
                                .flatten()
                                .filter_map(|e| e.ok())
                                .map(|e| e.path())
                                .filter(|f| f.is_file())
                                .filter(|f| !f.ends_with(UNDERSCORE_FILE_NAME));

                            let rendering_files = read_dir(path.join(RENDERING_DIRECTORY))
                                .into_iter()
                                .flatten()
                                .filter_map(|e| e.ok())
                                .map(|e| e.path())
                                .filter(|f| f.is_file());

                            results.insert(VirtualFileMapping::Directory {
                                root_file: underscore_file,
                                extra_files: BTreeSet::from_iter(
                                    sibling_files.chain(rendering_files),
                                ),
                            });
                        }
                        false => {
                            warn!("Found ")
                        }
                    }
                }
            }
            Some(results)
        }
        (true, false) => {
            warn!(
                "Unable to process `{}`. Expected a directory, but found a file instead.",
                TYPES_DIRECTORY
            );
            None
        }
        (false, _) => {
            warn!("Did not find the `{}` directory.", TYPES_DIRECTORY);
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
            assert_eq!(
                captured_logs[0].body,
                format!("Did not find the `{}` directory.", TYPES_DIRECTORY)
            );
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
                format!(
                    "Unable to process `{}`. Expected a directory, but found a file instead.",
                    TYPES_DIRECTORY
                )
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
        let nested_underscore_path = directory_type_dir.join(UNDERSCORE_FILE_NAME);
        std::fs::write(&nested_underscore_path, "").ok();
        let nested_description_path = directory_type_dir.join("description.txt");
        std::fs::write(&nested_description_path, "").ok();

        let another_directory_type_dir = dir.join("another_directory_type");
        std::fs::create_dir(&another_directory_type_dir).ok();
        let another_nested_underscore_path = another_directory_type_dir.join(UNDERSCORE_FILE_NAME);
        std::fs::write(&another_nested_underscore_path, "").ok();
        let nested_rendering_dir = another_directory_type_dir.join(RENDERING_DIRECTORY);
        std::fs::create_dir(&nested_rendering_dir).ok();
        let nested_template_path = nested_rendering_dir.join("txt.hjs");
        std::fs::write(&nested_template_path, "").ok();
        // Following should be ignored because type mapping only looks one level deep
        let unmapped_folder = another_directory_type_dir.join("ignored_folder");
        std::fs::create_dir(unmapped_folder).ok();
        let unmapped_folder_in_rendering = nested_rendering_dir.join("ignored_folder");
        std::fs::create_dir(unmapped_folder_in_rendering).ok();

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

    #[test]
    fn mapping_contents_fails_on_non_existing_directories() {
        testing_logger::setup();

        let empty_dir: PathBuf = testdir!();

        assert!(map_contents_directory(empty_dir.join("something")).is_none());
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(
                captured_logs[0].body,
                format!("Did not find the `{}` directory.", CONTENTS_DIRECTORY)
            );
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }

    #[test]
    fn mapping_contents_fails_on_non_directories() {
        testing_logger::setup();

        let dir: PathBuf = testdir!();
        let file = dir.join("file.txt");
        std::fs::write(&file, "something").ok();

        assert!(map_contents_directory(file).is_none());
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(
                captured_logs[0].body,
                format!(
                    "Unable to process `{}`. Expected a directory, but found a file instead.",
CONTENTS_DIRECTORY                )
            );
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }


}
