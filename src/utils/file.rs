use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};
const HEADER_MARKER: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
const HEADER_SIZE: usize = 4;
const VERSION_SIZE: usize = 4;
const LAYER_SIZE: usize = 4;

#[macro_export]
macro_rules! file_as_str {
    ($name: expr) => {{
        use std::io::Read;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($name)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        (file, buffer)
    }};
}

#[macro_export]
macro_rules! file_as_bytes {
    ($name: expr) => {{
        use std::io::Read;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($name)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        (file, buffer)
    }};
}
#[macro_export]
macro_rules! file_truncate {
    ($name: expr) => {{
        use std::io::Read;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open($name)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        (file, buffer)
    }};
}

#[macro_export]
macro_rules! config_line {
    ($keyword: expr, $arg: expr) => {{
        format!("{}={}", $keyword, $arg)
    }};
    ($keyword: expr, $arg: expr, $arg2: expr) => {{
        format!("{}={}={}", $keyword, $arg, $arg2)
    }};
    ($keyword: expr, $arg: expr, $arg2: expr, $arg3: expr) => {{
        format!("{}={}={}={}", $keyword, $arg, $arg2, $arg3)
    }};
}
#[macro_export]
macro_rules! parse_config_line {
    ($line: expr) => {{
        let parts: Vec<String> = $line.split('=').map(|w| w.trim().to_string()).collect();
        if parts.len() < 2 {
            None
        } else {
            Some(parts)
        }
    }};
}

#[macro_export]
macro_rules! crypted_file {
    ($path: expr) => {{
        CryptedFile::new($path)
    }};
}

use crate::encryption::{latest_encryption_version, EncryptionError};

#[derive(Debug, Clone)]
pub struct CryptedFile {
    pub path: PathBuf,
    pub read: OpenOptions,
    pub edit: OpenOptions,
}

impl CryptedFile {
    pub fn new(path: PathBuf) -> Result<Self, EncryptionError> {
        let common_options = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .clone();

        Ok(CryptedFile {
            path,
            edit: common_options.clone().truncate(true).clone(),
            read: common_options,
        })
    }
    pub fn from(mut self, other_file: &mut CryptedFile) -> Result<Self, EncryptionError> {
        let (layer, version) = other_file.header()?;

        let new_header = self.generate_header(layer, version)?;

        self.new_file_header(new_header)?;
        Ok(self)
    }

    fn reader(&self) -> Result<File, EncryptionError> {
        Ok(self.read.open(&self.path)?)
    }
    fn editor(&self) -> Result<File, EncryptionError> {
        Ok(self.edit.open(&self.path)?)
    }

    pub fn content_as_bytes(&self) -> Result<Vec<u8>, EncryptionError> {
        let mut file = self.reader()?;
        let mut content_as_bytes = Vec::new();
        file.read_to_end(&mut content_as_bytes)?;

        Ok(content_as_bytes)
    }
    // pub fn content_as_str(&mut self) -> Result<String, EncryptionError> {
    //     Ok(String::from_utf8_lossy(&self.content_as_bytes()?).to_string())
    // }

    pub fn encryption_layer(&self) -> Result<u32, EncryptionError> {
        Ok(self.header()?.0)
    }
    pub fn encryption_version(&self) -> Result<u32, EncryptionError> {
        Ok(self.header()?.1)
    }

    pub fn update_content(&mut self, new_content: Vec<u8>) -> Result<(), EncryptionError> {
        let (layer, version) = self.header()?;

        let mut new_content_with_header = Vec::new();

        if layer != 0 {
            let header_content = self.generate_header(layer, version)?;
            new_content_with_header.extend_from_slice(&header_content);
        }

        new_content_with_header.extend_from_slice(&new_content);
        self.editor()?.write_all(&new_content_with_header)?;

        Ok(())
    }

    fn update_header(&mut self, new_header: Vec<u8>) -> Result<(), EncryptionError> {
        let mut new_content = new_header;
        new_content.extend_from_slice(&self.main_file_content()?);

        self.editor()?.write_all(&new_content)?;

        Ok(())
    }
    fn new_file_header(&mut self, new_header: Vec<u8>) -> Result<(), EncryptionError> {
        self.editor()?.write_all(&new_header)?;

        Ok(())
    }

    pub fn increment_layer(&mut self) -> Result<(), EncryptionError> {
        let (layer, version) = self.header()?;
        let new_header = self.generate_header(layer + 1, version)?;
        self.update_header(new_header)?;
        Ok(())
    }
    pub fn decrement_layer(&mut self) -> Result<(), EncryptionError> {
        let (layer, version) = self.header()?;
        let new_header = self.generate_header(layer - 1, version)?;
        self.update_header(new_header)?;
        Ok(())
    }
    // pub fn set_version(&mut self, version: u32) -> Result<(), EncryptionError> {
    //     let (layer, _) = self.header()?;
    //     let new_header = self.generate_header(layer, version)?;

    //     self.update_header(new_header)?;
    //     Ok(())
    // }
    // pub fn increment_version(&mut self) -> Result<(), EncryptionError> {
    //     let (layer, version) = self.header()?;
    //     let new_header = self.generate_header(layer, version + 1)?;

    //     self.update_header(new_header)?;
    //     Ok(())
    // }

    pub fn header(&self) -> Result<(u32, u32), EncryptionError> {
        let content_as_bytes = self.content_as_bytes()?;

        if content_as_bytes.len() < HEADER_SIZE + VERSION_SIZE + LAYER_SIZE
            || &content_as_bytes[0..HEADER_SIZE] != HEADER_MARKER
        {
            return Ok((0, latest_encryption_version()?));
        } else {
            let version_bytes = &content_as_bytes[HEADER_SIZE..HEADER_SIZE + VERSION_SIZE];
            let layer_bytes = &content_as_bytes
                [HEADER_SIZE + VERSION_SIZE..HEADER_SIZE + VERSION_SIZE + LAYER_SIZE];

            let version = u32::from_be_bytes(version_bytes.try_into().unwrap());
            let layer = u32::from_be_bytes(layer_bytes.try_into().unwrap());

            Ok((layer, version))
        }
    }

    fn generate_header(&self, layer: u32, version: u32) -> Result<Vec<u8>, EncryptionError> {
        let mut header_content = Vec::new();

        let version_bytes: [u8; VERSION_SIZE] = version.to_be_bytes();
        let layer_bytes: [u8; LAYER_SIZE] = layer.to_be_bytes();
        header_content.extend_from_slice(&HEADER_MARKER);
        header_content.extend_from_slice(&version_bytes);
        header_content.extend_from_slice(&layer_bytes);

        Ok(header_content)
    }

    pub fn main_file_content(&self) -> Result<Vec<u8>, EncryptionError> {
        let content_as_bytes = self.content_as_bytes()?;

        if content_as_bytes.len() == 0 {
            return Err(EncryptionError::CannotProcessVoidFile);
        }

        if &content_as_bytes[0..HEADER_SIZE] != HEADER_MARKER {
            return Ok(content_as_bytes.to_vec());
        }

        if content_as_bytes.len() == HEADER_SIZE + VERSION_SIZE + LAYER_SIZE {
            return Ok(vec![]);
        }

        let file_content = match content_as_bytes.split_first() {
            Some((_, rest)) => &rest[HEADER_SIZE + VERSION_SIZE + LAYER_SIZE - 1..],
            None => return Err(EncryptionError::InvalidFileContent),
        };

        Ok(file_content.to_vec())
    }
}
