use crate::utils::GenericError;
use crate::{print_err, print_success};
use clap::ArgMatches;
use std::{fs, path::PathBuf};

pub fn copy_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if let Some(target) = sub_matches.get_one::<PathBuf>("TARGET") {
            match copy(filepath, target) {
                Ok(_) => print_success!("{:?} was copied in {:?} successfully", filepath, target),
                Err(e) => print_err!("(copy failed): {}", e),
            }
        }
    }
}

pub fn copy(source_path: &PathBuf, target: &PathBuf) -> Result<(), GenericError> {
    if source_path == target && source_path.is_file() {
        return Err(GenericError::Custom {
            message: "Cannot copy target inside itself".to_string(),
        });
    }

    if source_path.is_file() {
        // copy file into a designed dir or into another file
        let target = if target.is_dir() {
            target
                .to_path_buf()
                .join(
                    source_path
                        .file_name()
                        .ok_or_else(|| GenericError::Custom {
                            message: "Invalid source path".to_string(),
                        })?,
                )
        } else {
            target.to_path_buf()
        };

        fs::copy(source_path, &target)?;
    } else if source_path.is_dir() {
        // If the source_path is a directory, create the target directory if it doesn't exist
        if !target.exists() {
            fs::create_dir(&target)?;
        }

        // Get the entries (files and subdirectories) in the source directory
        let entries = fs::read_dir(source_path)?;

        // Recursively copy all entries to the target directory
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            // Generate the corresponding target path for the entry
            let target_path = target.join(entry.file_name());

            // Recursively copy the entry
            copy(&entry_path, &target_path)?;
        }
    } else {
        // If the source_path is neither a file nor a directory, return an error
        return Err(GenericError::Custom {
            message: "Invalid source path".to_string(),
        });
    }
    Ok(())
}
