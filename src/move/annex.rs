use std::{fs, path::PathBuf};

use crate::utils::GenericError;

pub fn mv(source_path: &PathBuf, target_dir: &PathBuf) -> Result<(), GenericError> {
    if source_path == target_dir {
        return Err(GenericError::Custom {
            message: "Source must be different from target".to_string(),
        });
    }

    // Check if the source path exists
    if !source_path.exists() {
        return Err(GenericError::Custom {
            message: "Source path does not exist".to_string(),
        });
    }

    if !target_dir.exists() {
        return Err(GenericError::Custom {
            message: "Target directory does not exist".to_string(),
        });
    }

    let target_path =
        target_dir.join(
            source_path
                .file_name()
                .ok_or_else(|| GenericError::Custom {
                    message: "Invalid source path".to_string(),
                })?,
        );
    if source_path.is_file() {
        fs::rename(source_path, &target_path).map_err(|err| GenericError::Custom {
            message: format!("File move error: {}", err),
        })?;
    } else if source_path.is_dir() {
        fs::rename(source_path, target_path).map_err(|err| GenericError::Custom {
            message: format!("Directory move error: {}", err),
        })?;
    } else {
        return Err(GenericError::Custom {
            message: "Invalid source path".to_string(),
        });
    }

    Ok(())
}
