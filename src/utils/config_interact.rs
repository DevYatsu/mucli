use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use crate::parse_config_line;

use super::GenericError;
use crate::utils::get_config_path;

#[macro_export]
macro_rules! config {
    () => {{
        use crate::Config;

        Config::new()
    }};
}

pub struct Config {
    pub file: File,
    buffer: String,
}
impl Config {
    pub fn new() -> Result<Self, GenericError> {
        let path = get_config_path()?;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(Self { file, buffer })
    }
    pub fn overwrite_content(&mut self, new_content: &[u8]) -> Result<(), GenericError> {
        self.file.seek(SeekFrom::Start(0))?;

        // Write the modified contents to the file
        self.file.write_all(new_content)?;

        // Truncate any remaining content after the new data
        self.file.set_len(new_content.len() as u64)?;

        Ok(())
    }

    pub fn set_key(&mut self, new_line: String) -> Result<(), GenericError> {
        writeln!(self.file, "{}", new_line)?;

        Ok(())
    }

    pub fn replace_key(&mut self, keyword: &str, mut new_line: String) -> Result<(), GenericError> {
        new_line.push('\n');
        // Create a new buffer with modified lines
        let modified_buffer = self
            .buffer
            .lines()
            .filter(|line| !line.starts_with(&format!("{}=", keyword)))
            .chain(std::iter::once(new_line.as_str()))
            .collect::<Vec<&str>>()
            .join("\n");

        self.overwrite_content(&modified_buffer.as_bytes())?;

        Ok(())
    }

    pub fn get_keys(&self, keyword: &str) -> Vec<String> {
        self.buffer
            .lines()
            .filter(|line| line.starts_with(&format!("{}{}", keyword, "=")))
            .map(|l| parse_config_line!(l).unwrap().into_iter().nth(1).unwrap())
            .collect::<Vec<String>>()
    }

    pub fn get_key(&self, keyword: &str) -> Result<Option<String>, GenericError> {
        for line in self.buffer.lines() {
            if line.starts_with(&format!("{}{}", keyword, "=")) {
                return Ok(parse_config_line!(line).unwrap().into_iter().nth(1));
            }
        }

        Ok(None)
    }

    pub fn filter_map_lines<F, T>(&self, f: F) -> Result<Vec<T>, GenericError>
    where
        F: FnMut(&str) -> Option<T>,
    {
        Ok(self.buffer.lines().filter_map(f).collect())
    }

    pub fn key_exists(&self, keyword: &str) -> Result<bool, GenericError> {
        if self
            .buffer
            .lines()
            .any(|line| line.starts_with(&format!("{}=", keyword)))
        {
            // Encryption key already exists, no need to write it again
            return Ok(true);
        }
        Ok(false)
    }
}

pub fn string_as_vec<T: std::str::FromStr>(string: &str) -> Result<Vec<T>, T::Err>
where
    T::Err: std::fmt::Debug,
{
    Ok(string
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|val| val.trim().parse::<T>().unwrap())
        .collect::<Vec<T>>())
}
pub fn vec_as_string<T: ToString>(vec: Vec<T>) -> String {
    format!(
        "[{}]",
        vec.into_iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}
