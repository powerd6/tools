use log::{warn};

use std::collections::BTreeSet;

use std::path::PathBuf;

use crate::vfs::CONTENTS_DIRECTORY;

use super::{get_paths_in_directory, VirtualFileMapping, UNDERSCORE_FILE_NAME};

pub(crate) fn map_contents_directory(
    contents_directory: &PathBuf,
) -> Option<BTreeSet<VirtualFileMapping>> {
    match (contents_directory.exists(), contents_directory.is_dir()) {
        (true, true) => {
            let mut results: BTreeSet<VirtualFileMapping> = BTreeSet::new();
            let root_underscore_file = contents_directory.join(UNDERSCORE_FILE_NAME);
            if root_underscore_file.exists() {
                results.insert(VirtualFileMapping::Directory {
                    root_file: root_underscore_file,
                    extra_files: BTreeSet::from_iter(
                        get_paths_in_directory(&contents_directory)
                            .filter(|e| e.is_file())
                            .filter(|f| !f.ends_with(UNDERSCORE_FILE_NAME)),
                    ),
                });
            } else {
                for path in get_paths_in_directory(&contents_directory) {
                    if path.is_file() {
                        results.insert(VirtualFileMapping::SingleFile { file_path: path });
                    } else if path.is_dir() {
                        let underscore_file = path.join(UNDERSCORE_FILE_NAME);
                        match underscore_file.exists() {
                            true => {
                                results.insert(VirtualFileMapping::Directory {
                                    root_file: underscore_file,
                                    extra_files: BTreeSet::from_iter(
                                        get_paths_in_directory(&path)
                                            .filter(|e| e.is_file())
                                            .filter(|f| !f.ends_with(UNDERSCORE_FILE_NAME)),
                                    ),
                                });
                            }
                            false => get_paths_in_directory(&path)
                                .filter(|e| e.is_file())
                                .for_each(|f| {
                                    results.insert(VirtualFileMapping::SingleFile { file_path: f });
                                }),
                        };
                        get_paths_in_directory(&path)
                            .filter(|e| e.is_dir())
                            .map(|d| map_contents_directory(&d))
                            .flatten()
                            .flatten()
                            .for_each(|v| {
                                results.insert(v);
                            });
                    }
                }
            }
            Some(results)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use log::Level;
    use pretty_assertions::assert_eq;
    use testdir::testdir;

    #[test]
    fn mapping_contents_fails_on_non_existing_directories() {
        testing_logger::setup();

        let empty_dir: PathBuf = testdir!();

        assert!(map_contents_directory(&empty_dir.join("something")).is_none());
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

        assert!(map_contents_directory(&file).is_none());
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(
                captured_logs[0].body,
                format!(
                    "Unable to process `{}`. Expected a directory, but found a file instead.",
                    CONTENTS_DIRECTORY
                )
            );
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
    }

    #[test]
    fn mapping_contents_reads_directory_correctly() {
        testing_logger::setup();

        let dir: PathBuf = testdir!();

        let first_content = dir.join("first.json");
        let second_content = dir.join("second");
        std::fs::create_dir(&second_content).ok();
        let second_underscore = second_content.join(UNDERSCORE_FILE_NAME);
        let second_description = second_content.join("description.txt");
        let grouping_dir = dir.join("grouping");
        std::fs::create_dir(&grouping_dir).ok();
        let grouping_summary = grouping_dir.join("summary.json");
        let first_grouping = grouping_dir.join("first");
        std::fs::create_dir(&first_grouping).ok();
        let first_grouping_underscore = first_grouping.join(UNDERSCORE_FILE_NAME);
        let first_grouping_description = first_grouping.join("description.txt");
        let second_grouping = grouping_dir.join("second.json");
        let third_grouping = grouping_dir.join("third");
        std::fs::create_dir(&third_grouping).ok();
        let third_grouping_underscore = first_grouping.join(UNDERSCORE_FILE_NAME);
        let third_grouping_description = first_grouping.join("description.txt");
        let third_sub_grouping = third_grouping.join("subgroup");
        std::fs::create_dir(&third_sub_grouping).ok();
        let fourth_content = third_sub_grouping.join("fourth.json");
        let fifth_content = third_sub_grouping.join("fifth.json");

        // Initialize all files with empty contents
        for file in vec![
            &first_content,
            &second_underscore,
            &second_description,
            &grouping_summary,
            &first_grouping_underscore,
            &first_grouping_description,
            &second_grouping,
            &third_grouping_underscore,
            &third_grouping_description,
            &fourth_content,
            &fifth_content,
        ] {
            std::fs::write(file, "").ok();
        }

        assert_eq!(
            map_contents_directory(&dir).unwrap(),
            BTreeSet::from([
                VirtualFileMapping::SingleFile {
                    file_path: first_content
                },
                VirtualFileMapping::Directory {
                    root_file: second_underscore,
                    extra_files: BTreeSet::from([second_description])
                },
                VirtualFileMapping::SingleFile {
                    file_path: grouping_summary
                },
                VirtualFileMapping::Directory {
                    root_file: first_grouping_underscore,
                    extra_files: BTreeSet::from([first_grouping_description])
                },
                VirtualFileMapping::SingleFile {
                    file_path: second_grouping
                },
                VirtualFileMapping::Directory {
                    root_file: third_grouping_underscore,
                    extra_files: BTreeSet::from([third_grouping_description])
                },
                VirtualFileMapping::SingleFile {
                    file_path: fourth_content
                },
                VirtualFileMapping::SingleFile {
                    file_path: fifth_content
                },
            ])
        )
    }
}
