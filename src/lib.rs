pub mod errors;

use std::path::PathBuf;

use crate::errors::ErrorCodes;

pub fn build_module(
    root_directory_path: PathBuf,
    output_file_name: String,
) -> Result<(), ErrorCodes> {
    println!("Building module from {}", root_directory_path.display());
    println!("Saving module to {}", output_file_name);
    Ok(())
}
