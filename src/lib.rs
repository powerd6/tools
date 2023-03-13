pub mod errors;

use std::path::PathBuf;

use log::{debug, info};

use crate::{errors::ErrorCodes, vfs::map_directory_to_module};

pub fn build_module(
    root_directory_path: PathBuf,
    output_file_name: String,
) -> Result<(), ErrorCodes> {
    info!("Building module from {}", root_directory_path.display());

    let vfs = map_directory_to_module(&root_directory_path);
    debug!("Mapped directory into VFS: {:#?}", vfs);

    info!("Saving module to {}", output_file_name);
    Ok(())
}

mod vfs;
