use std::{io::Write, path::PathBuf};

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
        use std::io::Read;

        let mut content_as_bytes = Vec::new();
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($path.to_path_buf())?
            .read_to_end(&mut content_as_bytes)?;

        // Convert the content to a string
        let content_as_str = String::from_utf8_lossy(&content_as_bytes).to_string();

        CryptedFile::new($path, content_as_bytes, content_as_str)
    }};
}

macro_rules! header {
    ($content_as_bytes: expr) => {{
        if $content_as_bytes.len() < HEADER_SIZE + VERSION_SIZE + LAYER_SIZE
            || &$content_as_bytes[0..HEADER_SIZE] != HEADER_MARKER
        {
            None
        } else {
            let version_bytes = &$content_as_bytes[HEADER_SIZE..HEADER_SIZE + VERSION_SIZE];
            let layer_bytes = &$content_as_bytes
                [HEADER_SIZE + VERSION_SIZE..HEADER_SIZE + VERSION_SIZE + LAYER_SIZE];

            let version = u32::from_be_bytes(version_bytes.try_into().unwrap());
            let layer = u32::from_be_bytes(layer_bytes.try_into().unwrap());

            Some((layer, version))
        }
    }};
}

use crate::encryption::EncryptionError;

pub struct CryptedFile {
    pub path: PathBuf,
    pub content_as_str: String,
    pub encryption_layer: u32,
    pub encryption_version: u32,
    pub content_as_bytes: Vec<u8>,
}

impl CryptedFile {
    pub fn new(
        path: PathBuf,
        content_as_bytes: Vec<u8>,
        content_as_str: String,
    ) -> Result<Self, EncryptionError> {
        let (encryption_layer, encryption_version) = match header!(&content_as_bytes) {
            Some((layer, version)) => {
                let latest_v = crate::encryption::latest_encryption_version()?;
                (layer, latest_v.min(version))
            }
            None => (0, crate::encryption::latest_encryption_version()?),
        };

        Ok(CryptedFile {
            content_as_str,
            content_as_bytes,
            path,
            encryption_layer,
            encryption_version,
        })
    }
    fn update_data(&mut self) -> Result<(), EncryptionError> {
        let CryptedFile {
            content_as_str,
            encryption_layer,
            encryption_version,
            content_as_bytes,
            path,
        } = crypted_file!(self.path.to_path_buf())?;

        self.content_as_str = content_as_str;
        self.encryption_layer = encryption_layer;
        self.encryption_version = encryption_version;
        self.content_as_bytes = content_as_bytes;

        Ok(())
    }

    pub fn update_file(&mut self, new_content: Vec<u8>) -> Result<(), EncryptionError> {    
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path)?;

        let mut new_content_with_header = Vec::new();
    println!("q {}", self.encryption_layer == 0);

        if self.encryption_layer == 0 {
            // Convert the version and layer to bytes and add the header marker
            let version_bytes: [u8; VERSION_SIZE] = self.encryption_version.to_be_bytes();
            let layer_bytes: [u8; LAYER_SIZE] = self.encryption_layer.to_be_bytes();
            new_content_with_header.extend_from_slice(&HEADER_MARKER);        
            new_content_with_header.extend_from_slice(&version_bytes);
            new_content_with_header.extend_from_slice(&layer_bytes);
        }

        new_content_with_header.extend_from_slice(&new_content);
        file.write_all(&new_content_with_header)?;

        self.update_data()?;
        Ok(())
    }

    pub fn increment_layer(&mut self) -> Result<(), EncryptionError> {
        self.encryption_layer += 1;              
        self.update_file(self.main_file_content()?)?;
        Ok(())
    }
    pub fn decrement_layer(&mut self) -> Result<(), EncryptionError> {
        self.encryption_layer -= 1;

        self.update_file(self.main_file_content()?)?;

        Ok(())
    }
    pub fn set_version(&mut self, version: u32) -> Result<(), EncryptionError> {
        self.encryption_version = version;

        self.update_file(self.main_file_content()?)?;

        Ok(())
    }
    pub fn increment_version(&mut self) -> Result<(), EncryptionError> {
        self.encryption_version += 1;

        self.update_file(self.main_file_content()?)?;

        Ok(())
    }

    pub fn main_file_content(&self) -> Result<Vec<u8>, EncryptionError> {
        if self.content_as_bytes.len() < HEADER_SIZE + VERSION_SIZE + LAYER_SIZE
            
        {
            return Err(EncryptionError::InvalidFileContent);
        }

        if &self.content_as_bytes[0..HEADER_SIZE] != HEADER_MARKER {
            return Ok(self.content_as_bytes.to_vec());
        }

        let file_content = match self.content_as_bytes.split_first() {
            Some((_, rest)) => &rest[HEADER_SIZE + VERSION_SIZE + LAYER_SIZE..],
            None => return Err(EncryptionError::InvalidFileContent),
        };

        Ok(file_content.to_vec())
    }
}
