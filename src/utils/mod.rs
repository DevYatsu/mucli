pub mod config_interact;
pub mod file;
pub mod terminal;

extern crate custom_error;
use std::{io::Error, num::ParseIntError};

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
