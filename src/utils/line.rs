use std::{
    fmt::{Debug, Display},
    io,
};

use custom_error::custom_error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

custom_error! {pub LineError
    Io{source: io::Error} = "{source}",
    Serde{source: serde_json::Error} = "{source}",
    InvalidFromArgument = "Argument to `Line::from() must be a string containing one '='`",
    DeserializeError{string: String} = "Could not deserialize: {string} into a key, value pair",
    FormatError = "Could not format Line, value must implement serialize",
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line<T> {
    pub key: String,
    pub value: T,
    // struct for lines that possess a struct key=value
}

impl<T: DeserializeOwned + Serialize> Display for Line<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={}",
            self.key,
            serde_json::to_string(&self.value).unwrap()
        )
    }
}

impl<T: for<'a> Deserialize<'a> + Serialize + DeserializeOwned> Line<T> {
    pub fn new(key: &str, value: T) -> Line<T> {
        Self {
            key: key.to_string(),
            value,
        }
    }

    pub fn from(string: &str) -> Result<Line<T>, LineError> {
        let mut parts = string.splitn(2, '=');
        let key = parts
            .next()
            .ok_or(LineError::InvalidFromArgument)?
            .to_string();
        let value_str = parts.next().ok_or(LineError::InvalidFromArgument)?;

        let value: T =
            serde_json::from_str(value_str).map_err(|_| LineError::DeserializeError {
                string: value_str.to_string(),
            })?;

        Ok(Self { key, value })
    }
}
impl<T: Serialize> Line<T> {
    pub fn format(&self) -> Result<String, LineError> {
        let value: String =
            serde_json::to_string(&self.value).map_err(|_| LineError::FormatError)?;

        Ok(format!("{}={}", self.key, value))
    }
}

impl<E> Line<Vec<E>> {
    pub fn add(&mut self, new_element: E) {
        self.value.push(new_element);
    }
    pub fn pop(&mut self) {
        self.value.pop();
    }
    pub fn set_new(&mut self, new_vec: Vec<E>) {
        self.value = new_vec;
    }
}
