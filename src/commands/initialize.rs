use std::path::PathBuf;

use clap::{Args, ValueEnum};
use log::{debug, trace};

use crate::file_system::FileSystem;

use super::Command;

#[derive(Args)]
/// Initializes a directory for a powerd6 module
pub(crate) struct Initialize {
    /// The path of the directory to be initialized
    #[arg(short, long, default_value = "./")]
    config: PathBuf,
    /// The format of the default and sample files to be created
    #[arg(short, long, value_enum, default_value_t=FileType::Json)]
    file_type: FileType,
}
#[derive(Clone, ValueEnum)]
enum FileType {
    Json,
    Yaml,
}

impl<F: FileSystem> Command<F> for Initialize {
    fn execute(&self, fs: &F) {
        trace!("Executing initialize");
        let root = self.initialize_root(fs);
        self.initialize_module_file(&root, fs);
        self.initialize_authors(&root, fs);
        debug!("Create authors directory");
        debug!("Create schema directory");
        debug!("Create contents directory");
    }
}

impl Initialize {
    fn initialize_root(&self, fs: &impl FileSystem) -> PathBuf {
        trace!("Initializing root directory");
        let root = if fs.dir_exists(&self.config) {
            self.config.clone()
        } else {
            debug!("Creating root directory {:?}", &self.config);
            fs.create_dir(&self.config).unwrap()
        };
        root
    }

    fn initialize_module_file(&self, root: &PathBuf, fs: &impl FileSystem) {
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

    fn initialize_authors(&self, root: &PathBuf, fs: &impl FileSystem) {
        trace!("Initializing authors directory");
        let dir = if fs.dir_exists(&root.join("authors")) {
            root.join("authors")
        } else {
            debug!("Creating authors directory");
            fs.create_dir(&root.join("authors")).unwrap()
        };
        let author_files = fs.get_dir_files(dir.as_ref());

        let pd6_array_file = &dir.join("_array.pd6");
        if !fs.file_exists(pd6_array_file) {
            debug!("Creating {:?}", pd6_array_file);
            fs.create_file(pd6_array_file, "This file is a flag that transforms each item in it into an element of an array element")
                .expect("Array file could not be created");
        }

        if author_files.is_none() {
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
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::file_system::MockFileSystem;

    use super::{FileType, Initialize};

    #[test]
    fn it_creates_root_directory_when_needed() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(false);
        mock_fs
            .expect_create_dir()
            .once()
            .returning(|p| Ok(PathBuf::from(p)));

        Initialize {
            config: PathBuf::new(),
            file_type: FileType::Json,
        }
        .initialize_root(&mock_fs);
    }
    #[test]
    fn it_uses_existing_root_directory() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_dir_exists().once().return_const(true);
        mock_fs.expect_create_dir().never();

        Initialize {
            config: PathBuf::new(),
            file_type: FileType::Json,
        }
        .initialize_root(&mock_fs);
    }

    #[test]
    fn it_creates_module_file() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_file_exists().once().return_const(false);
        mock_fs
            .expect_create_file()
            .once()
            .returning(|p, _| Ok(PathBuf::from(p)));

        Initialize {
            config: PathBuf::new(),
            file_type: FileType::Json,
        }
        .initialize_module_file(PathBuf::new(), &mock_fs);
    }

    #[test]
    fn it_uses_existing_module_file() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs.expect_file_exists().once().return_const(true);
        mock_fs.expect_create_file().never();

        Initialize {
            config: PathBuf::new(),
            file_type: FileType::Json,
        }
        .initialize_module_file(PathBuf::new(), &mock_fs);
    }
}
