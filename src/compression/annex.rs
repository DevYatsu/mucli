use std::path::PathBuf;

use zip_extensions::zip_create_from_directory;

use super::CompressionError;

pub fn create_zip(source_path: &PathBuf, output_path: &PathBuf) -> Result<(), CompressionError> {
    match zip_create_from_directory(&output_path, &source_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(CompressionError::Custom { src: e.to_string() }),
    }
}
