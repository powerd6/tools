
use std::{collections::BTreeSet, fs::read_dir, path::PathBuf};

use self::{contents::map_contents_directory, types::map_types_directory};

mod contents;
mod types;

#[derive(Debug)]
pub(crate) struct VirtualFileSystem {
    pub(crate) types: BTreeSet<VirtualFileMapping>,
    pub(crate) contents: BTreeSet<VirtualFileMapping>,
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
    root_directory_path: &PathBuf,
) -> VirtualFileSystem {
    VirtualFileSystem {
        types: map_types_directory(&root_directory_path.join(TYPES_DIRECTORY)).unwrap_or_default(),
        contents: map_contents_directory(&root_directory_path.join(CONTENTS_DIRECTORY)).unwrap_or_default(),
    }
}

fn get_paths_in_directory(directory_path: &PathBuf) -> impl Iterator<Item = PathBuf> {
    read_dir(directory_path)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
}
