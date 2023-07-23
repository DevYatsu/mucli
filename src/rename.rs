use std::{fs, path::PathBuf};

use crate::utils::GenericError;

pub fn rename(source_path: &PathBuf, name: &PathBuf) -> Result<(), GenericError> {
    if source_path == name {
        return Err(GenericError::Custom { message: format!( "Target is already named {:?}", name) })
    }

    if let Some(path_to_dir) = source_path.parent() {
        fs::rename(source_path, path_to_dir.join(name))?;
    } else {
        fs::rename(source_path, name)?;
    }
    Ok(())
}
