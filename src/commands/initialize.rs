use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};
use log::{debug, trace};

use crate::file_system::FileSystem;

use super::Command;

#[derive(Args, Debug)]
/// Initializes a directory for a powerd6 module
pub(crate) struct Initialize {
    /// The path of the directory to be initialized
    #[arg(short, long, default_value = "./")]
    config: PathBuf,
    /// The format of the default and sample files to be created
    #[arg(short, long, value_enum, default_value_t=FileType::Yaml)]
    file_type: FileType,
}
#[derive(Clone, ValueEnum, Debug)]
enum FileType {
    /// The more compact format, closer to what will be distributed
    Json,
    /// A more friendly and expressive format, easier to edit
    Yaml,
}

impl<F: FileSystem> Command<F> for Initialize {
    fn execute(&self, fs: &F) {
        trace!("Executing initialize: {:#?}", &self);
        let root = self.initialize_root(fs);
        self.initialize_module_file(&root, fs);
        self.initialize_authors(&root, fs);
        self.initialize_schema(&root, fs);
        self.initialize_content(&root, fs);
        trace!("Finished initialize")
    }
}

impl Initialize {
    fn initialize_root(&self, fs: &impl FileSystem) -> PathBuf {
        trace!("Initializing root directory");
        fs.create_dir_if_not_exists(&self.config).unwrap()
    }

    fn initialize_module_file(&self, root: &Path, fs: &impl FileSystem) {
        trace!("Initializing module.yaml");
        let (file_name, contents) = match self.file_type {
            FileType::Json => (
                "module.json",
                include_str!("../../fixtures/commands/initialize/module.json"),
            ),
            FileType::Yaml => (
                "module.yaml",
                include_str!("../../fixtures/commands/initialize/module.yaml"),
            ),
        };

        let module_file = &root.join(file_name);
        if !fs.file_exists(module_file) {
            debug!("Creating {:?}", module_file);
            fs.create_file(module_file, contents)
                // TODO: Refactor to remove the need for these `expect` statements
                .expect("Module file could not be created");
        }
    }

    fn initialize_authors(&self, root: &Path, fs: &impl FileSystem) {
        trace!("Initializing authors directory");
        let dir = fs.create_dir_if_not_exists(&root.join("authors")).unwrap();

        let dir_children = fs.get_dir_children(dir.as_ref());
        debug!(
            "Authors directory has {} files or subdirectories",
            dir_children.clone().map_or(0, |v| v.len())
        );

        fs.create_file_if_not_exists(
            &dir.join("_array.pd6"),
            include_str!("../../fixtures/commands/initialize/_array.pd6"),
        )
        .expect("Array file could not be created");

        if dir_children.is_none() {
            trace!("Creating sample author");
            let (file_name, contents) = match self.file_type {
                FileType::Json => (
                    "author.json",
                    include_str!("../../fixtures/commands/initialize/author.json"),
                ),
                FileType::Yaml => (
                    "author.yaml",
                    include_str!("../../fixtures/commands/initialize/author.yaml"),
                ),
            };
            fs.create_file(&dir.join(file_name), contents)
                .expect("Sample author file could not be created");
        }
    }

    fn initialize_schema(&self, root: &Path, fs: &impl FileSystem) {
        trace!("Initializing schema directory");
        let dir = fs.create_dir_if_not_exists(&root.join("schema")).unwrap();

        let dir_children = fs.get_dir_children(dir.as_ref());
        debug!(
            "Schema directory has {} files",
            dir_children.clone().map_or(0, |v| v.len())
        );

        if dir_children.is_none() {
            trace!("Creating sample schema");
            let (file_name, contents) = match self.file_type {
                FileType::Json => (
                    "sample.json",
                    include_str!("../../fixtures/commands/initialize/schema.json"),
                ),
                FileType::Yaml => (
                    "sample.yaml",
                    include_str!("../../fixtures/commands/initialize/schema.yaml"),
                ),
            };
            fs.create_file(&dir.join(file_name), contents)
                .expect("Sample schema file could not be created");
        }
    }

    fn initialize_content(&self, root: &Path, fs: &impl FileSystem) {
        trace!("Initializing content directory");
        let dir = fs.create_dir_if_not_exists(&root.join("content")).unwrap();

        let dir_children = fs.get_dir_children(dir.as_ref());
        debug!(
            "Content directory has {} files",
            dir_children.clone().map_or(0, |v| v.len())
        );

        if dir_children.is_none() {
            trace!("Creating sample content");
            let (file_name, contents) = match self.file_type {
                FileType::Json => (
                    "content.json",
                    include_str!("../../fixtures/commands/initialize/content.json"),
                ),
                FileType::Yaml => (
                    "content.yaml",
                    include_str!("../../fixtures/commands/initialize/content.yaml"),
                ),
            };
            fs.create_file(&dir.join(file_name), contents)
                .expect("Sample content file could not be created");
        }
    }
}
