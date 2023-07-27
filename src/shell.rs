use crate::utils::GenericError;
use crate::{print_err, print_success};
use clap::ArgMatches;
use std::{ffi::OsString, path::PathBuf, process::Command};

pub fn shell_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if !filepath.exists() {
            print_err!("Source path must be a valid path!");
            return;
        }
        if !filepath.is_file() {
            print_err!("Source path must be a file!");
            return;
        }

        if filepath.extension() != Some(&OsString::from("sh")) {
            print_err!("(invalid file): File must end with \"sh\" extension");
            return;
        }

        match execute_shell_script(filepath) {
            Ok(_) => print_success!("Shell script execution successful"),
            Err(e) => print_err!("(execution error): {}", e),
        };
    }
}

fn execute_shell_script(path: &PathBuf) -> Result<(), GenericError> {
    Command::new("sh").arg(path).status()?;

    Ok(())
}
