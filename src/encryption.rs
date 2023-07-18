use rand::RngCore;
use std::fs::{self, File, OpenOptions};
use std::io::{Error, Read, Seek, SeekFrom, Write};

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
const CONFIG_FILE: &str = "config.txt";

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub EncryptionError
    Io{source: Error} = "Unable to open file",
    NoKeyFound = "No key found in config.txt"
}

pub fn encrypt_file(input_path: &str, output_path: &str) -> Result<(), EncryptionError> {
    let key: Vec<u8> = get_encryption_key()?;

    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    // Read the contents of the input file into a buffer
    let mut input_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_data)?;
    todo!()
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
            return Ok(line[ENCRYPTION_KEYWORD.len() + 1..].as_bytes().to_vec());
        }
    }

    Err(EncryptionError::NoKeyFound)
}

pub fn encryption_key_exists() -> Result<bool, EncryptionError> {
    let buffer: String = fs::read_to_string(CONFIG_FILE)?;

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
    if !encryption_key_exists()? {
        set_encryption_key()?;
    }
    Ok(())
}

pub fn update_encryption_key() -> Result<(), EncryptionError> {
    if encryption_key_exists()? {
        // retrieve and decrypt all data
        // change encryption key and then rewrite all data with new encryption
        todo!();
        set_encryption_key()?;
    }
    Ok(())
}
