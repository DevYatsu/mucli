use rand::RngCore;
use simplecrypt::{decrypt, encrypt};
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::path::{PathBuf, Path};

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
const CONFIG_FILE: &str = "config.txt";

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub EncryptionError
    Io{source: Error} = "Unable to open file",
    NoKeyFound = "No key found in config.txt",
    DecryptionFailed{filename: String} = "Decryption of the \"{filename}\" failed",
    EncryptionFailed{filename: String} = "Encryption of the \"{filename}\" failed",
    ConfigNotFound = "config file not found or unreadable"
}

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let key: Vec<u8> = get_encryption_key()?;

    let mut input_file: File = File::open(input_path)?;
    let mut output_file: File = File::create(output_path)?;

    // Read the contents of the input file into a buffer
    let mut input_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let encrypted: Vec<u8> = encrypt(&input_data, &key);

    output_file.write_all(&encrypted)?;

    Ok(())
}

pub fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let key: Vec<u8> = get_encryption_key()?;

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

    let new_line = format!("{}={:?}", ENCRYPTION_KEYWORD, generate_encryption_key(32));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file

    let filtered_lines: Vec<_> = buffer
        .lines()
        .filter(|line| !line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)))
        .collect();

    file.set_len(0)?;

    for line in filtered_lines {
        writeln!(file, "{}", line)?;
    }

    writeln!(file, "{}", new_line)?;

    Ok(())
}

pub fn get_encryption_key() -> Result<Vec<u8>, EncryptionError> {
    let mut file = File::open(CONFIG_FILE)?;
    let mut buffer = String::with_capacity(256); // Set an initial capacity for the buffer

    file.read_to_string(&mut buffer)?;

    for line in buffer.lines() {
        if line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)) {
            let key: Vec<u8> = line[ENCRYPTION_KEYWORD.len() + 1..]
                .to_string()
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|c| c.trim().parse::<u8>().unwrap())
                .collect();
            println!("{:?}", key);
            return Ok(key);
        }
    }

    Err(EncryptionError::NoKeyFound)
}

pub fn encryption_key_exists() -> Result<bool, EncryptionError> {
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
        // retrieve and decrypt all data
        // change encryption key and then rewrite all data with new encryption
        set_encryption_key()?;
        todo!();
    }
    Ok(())
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