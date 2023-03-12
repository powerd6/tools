pub mod errors;

use std::path::PathBuf;

use crate::{errors::ErrorCodes, vfs::map_directory_to_module};

pub fn build_module(
    root_directory_path: PathBuf,
    output_file_name: String,
) -> Result<(), ErrorCodes> {
    println!("Building module from {}", root_directory_path.display());
    println!(
        "Mapped directory into VFS: {:#?}",
        map_directory_to_module(&root_directory_path)
    );
    println!("Saving module to {}", output_file_name);
    Ok(())
}

mod vfs;
