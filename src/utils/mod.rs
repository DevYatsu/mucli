pub mod config_interact;
pub mod file;
pub mod terminal;

extern crate custom_error;
use std::{io::Error, num::ParseIntError};

use custom_error::custom_error;

custom_error! {pub GenericError
    Io{source: Error} = "{source}",
    Format{source: ParseIntError} = "{source}",
    KeyNotFound{key: String, filename: String} = "Key \"{key}\" not found in {filename}.",
    Unknown = "unknown error"
}
