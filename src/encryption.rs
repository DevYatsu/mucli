use crate::utils::config_interact::vec_as_string;
use crate::{file_as_bytes, config_line};
use rand::RngCore;
use simplecrypt::{decrypt, encrypt, DecryptionError};
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
use crate::utils::{
    config_interact::{filter_and_map_lines, key_exists, set_key},
    terminal::arrow_progress,
    GenericError,
};

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub EncryptionError
    Io{source: Error} = "{source}",
    Generic{source: GenericError} = "{source}",
    Decrypt{source: DecryptionError} = "{source}",
    NoKeyFound = "No key found in config.txt",
    RetrievingKey = "Error retrieving encryption key.",
    NoVersionFound = "No version found in config.txt",
    NoVersionSet = "No version set for this file",
    VersionFormat = "Invalid version format found in config.txt",
    EncryptionFailed{filename: String} = "Encryption of \"{filename}\" failed",
    ConfigNotFound = "Config file not found or unreadable",
    UpdateBeforeInit = "Cannot update the key. Please init first.",
    CannotAccessFile{filename: String} = "Cannot access file \"{filename}\"",
    InvalidFileContent = "Target file must be a crypted file",
    DecryptNotCryptedFile = "Cannot decrypt a non-encrypted file",
    KeyUpdateFailed = "Impossible to update encryption key"
}

struct FileHeader {
    encryption_layer: u32,
    encryption_version: u32,
}

impl FileHeader {
    fn new(encryption_version: u32, encryption_layer: u32) -> Self {
        FileHeader {
            encryption_version,
            encryption_layer,
        }
    }
    fn increment_layer(&mut self) -> () {
        self.encryption_layer += 1;
    }
    fn decrement_layer(&mut self) -> () {
        self.encryption_layer -= 1;
    }
    fn set_version(&mut self, version: u32) -> () {
        self.encryption_version = version;
    }
}

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let (mut input_file, input_data) = file_as_bytes!(input_path);

    let mut file_header = match get_file_data(&mut input_file) {
        Ok(mut header) => {
            let latest_v = latest_encryption_version()?;
            if latest_v <= header.encryption_version {
                header.set_version(latest_v);
            }
            header.increment_layer();
            header
        }
        Err(EncryptionError::InvalidFileContent) => {
            FileHeader::new(latest_encryption_version()?, 0)
        }
        Err(e) => return Err(e),
    };

    let key = match nth_encription_key(file_header.encryption_version as usize) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::RetrievingKey),
    };

    let data = if file_header.encryption_layer > 0 {
        file_content(input_data)?
    } else {
        file_header.increment_layer();
        input_data
    };

    let mut encrypted: Vec<u8> = encrypt(&data, &key);
    set_file_data(&mut encrypted, file_header)?;

    let mut output_file: File = File::create(output_path)?;
    output_file.write_all(&encrypted)?;

    Ok(())
}

pub fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let (mut input_file, encrypted_data) = file_as_bytes!(input_path);

    let mut file_header = get_file_data(&mut input_file)?;

    if file_header.encryption_layer == 0 {
        return Err(EncryptionError::DecryptNotCryptedFile);
    }

    let key = match nth_encription_key(file_header.encryption_version as usize) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::RetrievingKey),
    };

    let crypted_content: Vec<u8> = file_content(encrypted_data)?;

    let mut decrypted_content: Vec<u8> = decrypt(&crypted_content, &key)?;

    file_header.decrement_layer();
    if file_header.encryption_layer > 0 {
        set_file_data(&mut decrypted_content, file_header)?;
    }

    let mut output_file = File::create(&output_path)?;
    // set_file_data(&mut output_file, file_version)?;
    output_file.write_all(&decrypted_content)?;

    Ok(())
}

pub fn generate_encryption_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn set_encryption_key() -> Result<(), EncryptionError> {
    let version = match latest_encryption_version() {
        Ok(val) => val + 1,
        Err(_) => 0,
    };

    let new_line = config_line!(ENCRYPTION_KEYWORD, version, vec_as_string(generate_encryption_key(32)));

    set_key(new_line)?;

    Ok(())
}

fn encryption_keys() -> Result<Vec<Vec<u8>>, EncryptionError> {
    let mut filtered_lines =
        filter_and_map_lines(ENCRYPTION_KEYWORD, |line| parse_encryption_key_line(line))?;

    if filtered_lines.is_empty() {
        return Err(EncryptionError::NoKeyFound);
    }

    filtered_lines.sort_by_key(|(key, _)| *key);

    let encryption_keys = filtered_lines.into_iter().map(|(_, vec)| vec).collect();

    Ok(encryption_keys)
}

fn parse_encryption_key_line(line: &str) -> (u32, Vec<u8>) {
    let mut iterator = line.split('=');
    iterator.next();
    let key_value = iterator.next().unwrap().trim();
    let key = key_value.parse().unwrap();

    let raw_values = iterator
        .next()
        .unwrap()
        .trim()
        .trim_matches(|c| c == '[' || c == ']');

    let values: Vec<u8> = raw_values
        .split(',')
        .map(|val| val.trim().parse::<u8>().unwrap())
        .collect();

    (key, values)
}

pub fn init_encryption_key() -> Result<(), EncryptionError> {
    match key_exists(ENCRYPTION_KEYWORD) {
        Ok(val) => {
            if let false = val {
                set_encryption_key()?
            }
            Ok(())
        }
        Err(_) => Err(EncryptionError::ConfigNotFound),
    }
}

pub fn update_encryption_key() -> Result<(), EncryptionError> {
    match key_exists(ENCRYPTION_KEYWORD) {
        Ok(val) => {
            if let true = val {
                set_encryption_key()?
            } else {
                return Err(EncryptionError::KeyUpdateFailed);
            }
            Ok(())
        }
        Err(_) => Err(EncryptionError::ConfigNotFound),
    }
}

pub fn update_file_encryption_key(filepath: &PathBuf) -> Result<(), EncryptionError> {
    let mut file = File::open(filepath)?;
    let initial_layer = get_file_data(&mut file)?.encryption_layer;
    let pb = arrow_progress((initial_layer * 2) as u64);
    pb.set_prefix("Generating key...");

    // first we decrypt the file
    while let Ok(_) = get_file_data(&mut file) {
        decrypt_file(filepath, filepath)?;
        pb.inc(1);
    }

    // we encrypt it again with newly generated key
    let mut layer = 0;
    while layer < initial_layer {
        encrypt_file(filepath, filepath)?;
        layer += 1;
        pb.inc(1);
    }

    pb.finish_with_message("===> Key updated");
    thread::sleep(Duration::from_millis(500));
    pb.finish_and_clear();
    Ok(())
}

fn nth_encription_key(index: usize) -> Result<Vec<u8>, EncryptionError> {
    let keys = encryption_keys()?;
    let key = keys[index].clone();

    Ok(key)
}

fn latest_encryption_version() -> Result<u32, EncryptionError> {
    let mut filtered_lines: Vec<u32> = filter_and_map_lines(ENCRYPTION_KEYWORD, |line| {
        line.split('=')
            .nth(1)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap()
    })?;

    if filtered_lines.len() == 0 {
        return Err(EncryptionError::NoVersionFound);
    }

    filtered_lines.sort_by(|a, b| b.cmp(a));

    Ok(filtered_lines[0])
}

const HEADER_SIZE: usize = 4;
const VERSION_SIZE: usize = 4;
const LAYER_SIZE: usize = 4; // encryption layer, how many times the file was encrypted
fn set_file_data(data: &mut Vec<u8>, file_header: FileHeader) -> Result<(), EncryptionError> {
    // Convert the version to bytes to write to the file
    let version_bytes: [u8; VERSION_SIZE] = file_header.encryption_version.to_be_bytes();
    let header_marker: [u8; HEADER_SIZE] = [0xAA, 0xBB, 0xCC, 0xDD];
    let layer_bytes = file_header.encryption_layer.to_be_bytes();

    // Insert the header marker and version at the start of the vector
    data.splice(0..0, layer_bytes.iter().cloned());
    data.splice(0..0, version_bytes.iter().cloned());
    data.splice(0..0, header_marker.iter().cloned());

    Ok(())
}

fn get_file_data(file: &mut File) -> Result<FileHeader, EncryptionError> {
    let mut header_marker = [0u8; HEADER_SIZE];
    let mut version_bytes = [0u8; VERSION_SIZE];
    let mut layer_bytes = [0u8; LAYER_SIZE];

    // Save the current cursor position
    let current_pos = file.seek(SeekFrom::Current(0))?;

    // Move the cursor to the start of the file
    file.seek(SeekFrom::Start(0))?;

    // Read the header marker and version bytes from the file
    file.read_exact(&mut header_marker)?;
    if header_marker != [0xAA, 0xBB, 0xCC, 0xDD] {
        file.seek(SeekFrom::Start(current_pos))?;
        return Err(EncryptionError::InvalidFileContent);
    }

    file.read_exact(&mut version_bytes)?;
    let encryption_version = u32::from_be_bytes(version_bytes);

    file.read_exact(&mut layer_bytes)?;
    let encryption_layer = u32::from_be_bytes(layer_bytes);

    // Restore the original cursor position
    file.seek(SeekFrom::Start(current_pos))?;

    Ok(FileHeader {
        encryption_layer,
        encryption_version,
    })
}

fn file_content(mut decrypted: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
    if decrypted.len() < HEADER_SIZE + VERSION_SIZE + LAYER_SIZE {
        return Err(EncryptionError::InvalidFileContent);
    }

    // Split off the header and version bytes from the decrypted content
    let file_content: Vec<u8> = decrypted.split_off(HEADER_SIZE + VERSION_SIZE + LAYER_SIZE);
    Ok(file_content)
}

pub fn encrypted_file_path(file_path: &Path, current_dir: &Path) -> PathBuf {
    let mut file_name: String = file_path.file_name().unwrap().to_string_lossy().to_string();
    file_name = format!("enc.{}", file_name);

    let output_path = current_dir.join(file_name);
    output_path
}
pub fn decrypted_file_path(file_path: &Path, current_dir: &Path) -> PathBuf {
    let mut file_name: String = file_path.file_name().unwrap().to_string_lossy().to_string();
    if file_name.starts_with("enc.") {
        file_name = file_name[4..].to_string();
    }
    let output_path = current_dir.join(file_name);

    output_path
}
