use rand::RngCore;
use simplecrypt::{decrypt, encrypt};
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
const CONFIG_FILE: &str = "config.txt";

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub EncryptionError
    Io{source: Error} = "Unable to open file",
    NoKeyFound = "No key found in config.txt",
    RetrievingKey = "Error retrieving encryption key.",
    NoVersionFound = "No version found in config.txt",
    NoVersionSet = "No version set for this file",
    VersionFormat = "Invalid version format found in config.txt",
    DecryptionFailed{filename: String} = "Decryption of \"{filename}\" failed",
    EncryptionFailed{filename: String} = "Encryption of \"{filename}\" failed",
    ConfigNotFound = "Config file not found or unreadable",
    UpdateBeforeInit = "Cannot update the key. Please init first.",
    CannotAccessFile{filename: String} = "Cannot access file \"{filename}\"",
    InvalidFileContent = "Decrypted file as an invalid content."
}

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let mut input_file: File = File::open(input_path)?;
    let mut output_file: File = File::create(output_path)?;

    let version = match get_file_version(&mut input_file) {
        Ok(v) => {
            let latest_v = latest_encryption_version()?;
            if latest_v > v {
                v
            } else {
                latest_v
            }
        }
        Err(EncryptionError::NoVersionSet) => latest_encryption_version()?,
        Err(e) => return Err(e),
    };
    let key = match nth_encription_key(version as usize) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::RetrievingKey),
    };

    // Read the contents of the input file into a buffer
    let mut input_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let mut encrypted: Vec<u8> = encrypt(&input_data, &key);

    set_file_version(&mut encrypted, version)?;
    output_file.write_all(&encrypted)?;

    Ok(())
}

pub fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let mut input_file = File::open(&input_path)?;
    let mut output_file = File::create(&output_path)?;

    let file_version = get_file_version(&mut input_file)?;

    let key = match nth_encription_key(file_version as usize) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::RetrievingKey),
    };
    
    // Read the contents of the input file into a buffer
    let mut encrypted_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut encrypted_data)?;
    
    let crypted_content = file_content(encrypted_data)?;
    
    let decrypted_content: Vec<u8> = match decrypt(&crypted_content, &key) {
        Ok(result) => result,
        Err(_) => {
            return Err(EncryptionError::DecryptionFailed {
                filename: input_path.to_str().unwrap().to_owned(),
            })
        }
    };

    // set_file_version(&mut output_file, file_version)?;
    output_file.write_all(&decrypted_content)?;

    Ok(())
}

fn generate_encryption_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn set_encryption_key() -> Result<(), EncryptionError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(CONFIG_FILE)?;

    let version = match latest_encryption_version() {
        Ok(val) => val + 1,
        Err(_) => 0,
    };

    let new_line = format!(
        "{}={}={:?}",
        ENCRYPTION_KEYWORD,
        version,
        generate_encryption_key(32)
    );

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file

    file.set_len(0)?;

    for line in buffer.lines() {
        writeln!(file, "{}", line)?;
    }

    writeln!(file, "{}", new_line)?;

    Ok(())
}

fn encryption_keys() -> Result<Vec<Vec<u8>>, EncryptionError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(CONFIG_FILE)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut filtered_lines = extract_filtered_lines(&buffer)?;

    if filtered_lines.is_empty() {
        return Err(EncryptionError::NoKeyFound);
    }

    filtered_lines.sort_by_key(|(key, _)| *key);

    let encryption_keys = filtered_lines.into_iter().map(|(_, vec)| vec).collect();

    Ok(encryption_keys)
}

fn extract_filtered_lines(buffer: &str) -> Result<Vec<(u32, Vec<u8>)>, EncryptionError> {
    let filtered_lines: Vec<(u32, Vec<u8>)> = buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)))
        .map(|line| parse_encryption_key_line(line))
        .collect();

    Ok(filtered_lines)
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

fn encryption_key_exists() -> Result<bool, EncryptionError> {
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    if buffer
        .lines()
        .any(|line| line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)))
    {
        // Encryption key already exists, no need to write it again
        return Ok(true);
    }
    Ok(false)
}

pub fn init_encryption_key() -> Result<(), EncryptionError> {
    match encryption_key_exists() {
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
    match encryption_key_exists() {
        Ok(val) => {
            if let true = val {
                set_encryption_key()?
            } else {
                println!("Impossible to update encryption key.")
            }
            Ok(())
        }
        Err(_) => Err(EncryptionError::ConfigNotFound),
    }
}

fn nth_encription_key(index: usize) -> Result<Vec<u8>, EncryptionError> {
    let keys = encryption_keys()?;
    let key = keys[index].clone();

    Ok(key)
}

fn latest_encryption_version() -> Result<u32, EncryptionError> {
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    let mut filtered_lines: Vec<u32> = buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)))
        .map(|line| {
            line.split('=')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    if filtered_lines.len() == 0 {
        return Err(EncryptionError::NoVersionFound);
    }

    filtered_lines.sort_by(|a, b| b.cmp(a));

    Ok(filtered_lines[0])
}

const HEADER_SIZE: usize = 4;
const VERSION_SIZE: usize = 4;
fn set_file_version(data: &mut Vec<u8>, version: u32) -> Result<(), EncryptionError> {
    // Convert the version to bytes to write to the file
    let version_bytes: [u8; VERSION_SIZE] = version.to_be_bytes();
    let header_marker: [u8; HEADER_SIZE] = [0xAA, 0xBB, 0xCC, 0xDD];

    // Insert the header marker and version at the start of the vector
    data.splice(0..0, version_bytes.iter().cloned());
    data.splice(0..0, header_marker.iter().cloned());

    Ok(())
}

fn get_file_version(file: &mut File) -> Result<u32, EncryptionError> {
    let mut header_marker = [0u8; HEADER_SIZE];
    let mut version_bytes = [0u8; VERSION_SIZE];

    // Save the current cursor position
    let current_pos = file.seek(SeekFrom::Current(0))?;

    // Move the cursor to the start of the file
    file.seek(SeekFrom::Start(0))?;

    // Read the header marker and version bytes from the file
    file.read_exact(&mut header_marker)?;
    if header_marker != [0xAA, 0xBB, 0xCC, 0xDD] {
        file.seek(SeekFrom::Start(current_pos))?;
        return Err(EncryptionError::NoVersionSet);
    }

    file.read_exact(&mut version_bytes)?;
    let version = u32::from_be_bytes(version_bytes);

    // Restore the original cursor position
    file.seek(SeekFrom::Start(current_pos))?;

    Ok(version)
}

fn file_content(mut decrypted: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
    if decrypted.len() < HEADER_SIZE + VERSION_SIZE {
        return Err(EncryptionError::InvalidFileContent);
    }

    // Split off the header and version bytes from the decrypted content
    let file_content = decrypted.split_off(HEADER_SIZE + VERSION_SIZE);
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
