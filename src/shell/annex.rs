use std::{path::PathBuf, process::Command};

use crate::utils::GenericError;

pub fn execute_shell_script(path: &PathBuf) -> Result<(), GenericError> {
    Command::new("sh").arg(path).status()?;

    Ok(())
}
