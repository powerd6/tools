use log::warn;

use std::collections::BTreeSet;

use std::path::PathBuf;

use crate::vfs::TYPES_DIRECTORY;

use super::{
    get_paths_in_directory, VirtualFileMapping, RENDERING_DIRECTORY, UNDERSCORE_FILE_NAME,
};

pub(crate) fn map_types_directory(
    types_directory_path: &PathBuf,
) -> Option<BTreeSet<VirtualFileMapping>> {
    match (types_directory_path.exists(), types_directory_path.is_dir()) {
        (true, true) => {
            let mut results: BTreeSet<VirtualFileMapping> = BTreeSet::new();

            for path in get_paths_in_directory(&types_directory_path) {
                if path.is_file() {
                    results.insert(VirtualFileMapping::SingleFile { file_path: path });
                } else if path.is_dir() {
                    let underscore_file = path.join(UNDERSCORE_FILE_NAME);
                    match underscore_file.exists() {
                        true => {
                            let sibling_files = get_paths_in_directory(&path)
                                .filter(|f| f.is_file())
                                .filter(|f| !f.ends_with(UNDERSCORE_FILE_NAME));

                            let rendering_files =
                                get_paths_in_directory(&path.join(RENDERING_DIRECTORY))
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

        assert!(map_types_directory(&empty_dir.join("something")).is_none());
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

        assert!(map_types_directory(&file).is_none());
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

        let first_type = dir.join("single_type.json");
        let second_type = dir.join("second_type");
        std::fs::create_dir(&second_type).ok();
        let second_underscore = second_type.join(UNDERSCORE_FILE_NAME);
        let second_description = second_type.join("description.txt");
        let third_type = dir.join("third_type");
        std::fs::create_dir(&third_type).ok();
        let third_underscore = third_type.join(UNDERSCORE_FILE_NAME);
        let third_rendering_dir = third_type.join(RENDERING_DIRECTORY);
        std::fs::create_dir(&third_rendering_dir).ok();
        let third_rendering_txt = third_rendering_dir.join("txt.hjs");
        
        // Following should be ignored because type mapping only looks one level deep
        let ignored_dir = third_type.join("ignored_folder");
        std::fs::create_dir(ignored_dir).ok();
        let ignored_file = third_rendering_dir.join("ignored_folder");
        std::fs::create_dir(ignored_file).ok();

        // Initialize all files with empty contents
        for file in vec![
            &first_type,
            &second_underscore,
            &second_description,
            &third_underscore,
            &third_rendering_txt,
        ] {
            std::fs::write(file, "").ok();
        }

        assert_eq!(
            map_types_directory(&dir).unwrap(),
            BTreeSet::from([
                VirtualFileMapping::SingleFile {
                    file_path: first_type
                },
                VirtualFileMapping::Directory {
                    root_file: second_underscore,
                    extra_files: BTreeSet::from([second_description])
                },
                VirtualFileMapping::Directory {
                    root_file: third_underscore,
                    extra_files: BTreeSet::from([third_rendering_txt])
                }
            ])
        )
    }
}
