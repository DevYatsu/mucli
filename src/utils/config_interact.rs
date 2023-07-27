use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use super::{line::Line, GenericError};
use crate::utils::get_config_path;

pub struct Config {
    pub file: File,
    buffer: String,
}
impl Config {
    pub fn open() -> Result<Self, GenericError> {
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

    pub fn set_line<T: serde::Serialize>(&mut self, new_line: Line<T>) -> Result<(), GenericError> {
        writeln!(self.file, "{}", new_line.format()?)?;

        Ok(())
    }

    pub fn replace_key<T: serde::Serialize>(
        &mut self,
        new_line: Line<T>,
    ) -> Result<(), GenericError> {
        // Create a new buffer with modified lines
        let modified_buffer = self
            .buffer
            .lines()
            .filter(|line| !line.starts_with(&format!("{}=", new_line.key)))
            .chain(std::iter::once(new_line.format()?.as_str()))
            .collect::<Vec<&str>>()
            .join("\n");

        self.overwrite_content(&modified_buffer.as_bytes())?;

        Ok(())
    }

    pub fn get_line(&self, keyword: &str) -> Option<String> {
        for line in self.buffer.lines() {
            if line.starts_with(&format!("{}{}", keyword, "=")) {
                return Some(line.to_owned());
            }
        }

        None
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
