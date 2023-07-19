use rand::RngCore;
use simplecrypt::{decrypt, encrypt};
use std::fs::{self, File, OpenOptions};
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
const CONFIG_FILE: &str = "config.txt";
const VERSION_MASK: u32 = 0b1111_0000; // Mask to isolate the version bits

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub EncryptionError
    Io{source: Error} = "Unable to open file",
    NoKeyFound = "No key found in config.txt",
    NoVersionFound = "No version found in config.txt",
    VersionFormat = "Invalid version format found in config.txt",
    DecryptionFailed{filename: String} = "Decryption of the \"{filename}\" failed",
    EncryptionFailed{filename: String} = "Encryption of the \"{filename}\" failed",
    ConfigNotFound = "config file not found or unreadable"
}

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let key = latest_encription_key()?;
    let version = latest_encryption_version()?;

    let mut input_file: File = File::open(input_path)?;
    let mut output_file: File = File::create(output_path)?;

    // Read the contents of the input file into a buffer
    let mut input_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let encrypted: Vec<u8> = encrypt(&input_data, &key);

    output_file.write_all(&encrypted)?;

    set_file_version(&output_path, version)?;

    Ok(())
}

pub fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let file_version = get_file_version(&input_path)?;
    let key = nth_encription_key(file_version as usize)?;

    let mut input_file = File::open(&input_path)?;
    let mut output_file = File::create(&output_path)?;

    // Read the contents of the input file into a buffer
    let mut encrypted_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut encrypted_data)?;

    let decrypted: Vec<u8> = match decrypt(&encrypted_data, &key) {
        Ok(result) => result,
        Err(_) => {
            return Err(EncryptionError::DecryptionFailed {
                filename: input_path.to_str().unwrap().to_owned(),
            })
        }
    };

    output_file.write_all(&decrypted)?;

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

    let mut filtered_lines: Vec<(u32, Vec<u8>)> = buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)))
        .map(|line| {
            let mut iterator = line.split('=');
            return (
                iterator.nth(1).unwrap().trim().parse::<u32>().unwrap(),
                iterator
                    .nth(2)
                    .unwrap()
                    .trim()
                    .trim_matches(|c| c == '[' || c == ']')
                    .split(',')
                    .map(|val| val.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
        })
        .collect();

    if filtered_lines.is_empty() {
        return Err(EncryptionError::NoKeyFound);
    }

    filtered_lines.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(filtered_lines
        .into_iter()
        .map(|(_, vec)| vec)
        .collect::<Vec<Vec<u8>>>())
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
    if let Ok(true) = encryption_key_exists() {
        // find a simple way to recognize the version the file was encrypted with
        set_encryption_key()?;
        todo!();
    }
    Ok(())
}

fn latest_encription_key() -> Result<Vec<u8>, EncryptionError> {
    let keys = encryption_keys()?;
    let key = keys[keys.len() - 1].clone();

    Ok(key)
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

fn set_file_version(file_path: &PathBuf, version: u32) -> Result<(), EncryptionError> {
    let metadata = fs::metadata(file_path)?;
    let mut permissions = metadata.permissions();

    let mode = permissions.mode() & !VERSION_MASK; // Clear existing version bits

    let version_bits: u32 = (version) << 4; // Shift the version to the appropriate position
    let new_mode = mode | version_bits; // Combine with existing mode bits

    permissions.set_mode(new_mode);
    fs::set_permissions(file_path, permissions)?;

    Ok(())
}

fn get_file_version(file_path: &PathBuf) -> Result<u32, EncryptionError> {
    let metadata = fs::metadata(file_path)?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    let version_bits = (mode & VERSION_MASK) >> 4; // Extract the version bits
    let version = version_bits as u32;

    Ok(version)
}

pub fn encrypted_file_path(file_path: &Path, current_dir: &Path) -> PathBuf {
    let mut file_name: String = file_path.file_name().unwrap().to_string_lossy().to_string();
    file_name = format!("enc.{}", file_name);

    let output_path = current_dir.join(file_name);
    println!("{:?}", output_path);
    output_path
}
pub fn decrypted_file_path(file_path: &Path, current_dir: &Path) -> PathBuf {
    let mut file_name: String = file_path.file_name().unwrap().to_string_lossy().to_string();
    if file_name.starts_with("enc.") {
        file_name = file_name[4..].to_string();
    }
    let output_path = current_dir.join(file_name);
    println!("{:?}", output_path);

    output_path
}
