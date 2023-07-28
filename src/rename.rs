use crate::utils::GenericError;

use std::{fs, path::PathBuf};

use clap::ArgMatches;

use crate::{print_err, print_success};

pub fn rename_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if let Some(new_name) = sub_matches.get_one::<PathBuf>("NAME") {
            if new_name.to_string_lossy().contains("/") {
                print_err!("(renaming failed): Invalid new name");
                return;
            }

            match rename(filepath, new_name) {
                Ok(_) => {
                    print_success!("{:?} renamed {:?} successfully", filepath, new_name)
                }
                Err(e) => print_err!("(renaming failed): {}", e),
            }
        }
    }
}

pub fn rename(source_path: &PathBuf, name: &PathBuf) -> Result<(), GenericError> {
    if source_path == name {
        return Err(GenericError::Custom {
            message: format!("Target is already named {:?}", name),
        });
    }

    if let Some(path_to_dir) = source_path.parent() {
        fs::rename(source_path, path_to_dir.join(name))?;
    } else {
        fs::rename(source_path, name)?;
    }
    Ok(())
}
