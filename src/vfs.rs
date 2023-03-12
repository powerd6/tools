/// VFS (Virtual File System) is an abstraction on top of the actual file system.
/// It serves as a layer of abstraction and allows for manipulation of files without touching the disk.

use crate::errors::ErrorCodes;
use std::{collections::BTreeSet, fs::read_dir, path::PathBuf};

mod contents;
mod types;

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
    root_directory_path: &PathBuf,
) -> Result<VirtualFileSystem, ErrorCodes> {
    let types_directory: PathBuf = root_directory_path.join(TYPES_DIRECTORY);
    let contents_directory: PathBuf = root_directory_path.join(CONTENTS_DIRECTORY);

    Ok(VirtualFileSystem {
        types: types::map_types_directory(&types_directory),
        contents: contents::map_contents_directory(&contents_directory),
    })
}

fn get_paths_in_directory(directory_path: &PathBuf) -> impl Iterator<Item = PathBuf> {
    read_dir(directory_path)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
}
