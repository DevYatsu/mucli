pub mod config_interact;
pub mod file;
pub mod terminal;

extern crate custom_error;
use std::{io::Error, num::ParseIntError, path::PathBuf};

use custom_error::custom_error;
use rand::RngCore;

custom_error! {pub GenericError
    Io{source: Error} = "{source}",
    Format{source: ParseIntError} = "{source}",
    KeyNotFound{key: String} = "Key \"{key}\" not found in config file.",
    Unknown = "unknown error",
    Custom{message: String} = "{message}"
}

pub fn generate_encryption_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
pub fn get_config_path() -> Result<PathBuf, GenericError> {
    let home_dir = dirs::home_dir().ok_or(GenericError::Custom { message: "Cannot access home dir".to_string() })?;
    let config_path = home_dir.join("config.txt");
    Ok(config_path)
}
