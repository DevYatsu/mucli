pub mod config_interact;
pub mod file;
pub mod line;
pub mod terminal;

extern crate custom_error;
use std::{io::Error, num::ParseIntError, path::PathBuf};

use custom_error::custom_error;
use rand::RngCore;

use self::line::LineError;

custom_error! {pub GenericError
    Io{source: Error} = "{source}",
    Line{source: LineError} = "{source}",
    Format{source: ParseIntError} = "{source}",
    ReqWest{source: reqwest::Error} = "{source}",
    Deserialize{source: serde_json::Error} = "{source}",
    KeyNotFound{key: String} = "Key \"{key}\" not found in config file.",
    Unknown = "unknown error",
    Custom{message: String} = "{message}",
}

pub fn generate_encryption_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

pub fn get_config_path() -> Result<PathBuf, GenericError> {
    let home_dir = dirs::home_dir().ok_or(GenericError::Custom {
        message: "Cannot access home dir".to_string(),
    })?;
    let config_path = home_dir.join("mucli_config.txt");
    Ok(config_path)
}
