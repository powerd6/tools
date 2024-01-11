use std::path::PathBuf;

use clap::{arg, Args};
use log::trace;

use crate::{file_system::{FileSystem, formats::Format}, module::Module};

use super::Command;

#[derive(Args, Debug)]
/// Assembles a directory into a powerd6 module
pub(crate) struct Assemble {
    /// The path of the directory to be processed
    #[arg(short, long, default_value = "./")]
    config: PathBuf,
}

impl<F: FileSystem> Command<F> for Assemble {
    fn execute(&self, fs: &F) {
        trace!("Executing assemble: {:#?}", &self);
        let module = Module::new();
        self.get_module_information(fs, module);
        trace!("Process author directory");
        trace!("Process content directory");
        trace!("Process schema directory");
        trace!("Finished assemble")
    }
}

impl Assemble {
    fn get_module_information(&self, fs: &impl FileSystem, module: Module) {
        trace!("Process module information");
        if let Some(module_file) = fs.has_file_named(&self.config, "module") {
            let format = Format::from(module_file.clone());
            trace!("Found {:?}. Using {:?} format", module_file, format);
            module.set_module_information(format.get_data(&module_file));
        }
    }
}
