use crate::utils::file::CryptedFile;
use crate::Config;
use crate::{config, config_line, crypted_file, parse_config_line};
use indicatif::ProgressBar;
use rand::RngCore;
use simplecrypt::{decrypt, encrypt, DecryptionError};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
use crate::utils::{config_interact::vec_as_string, terminal::arrow_progress, GenericError};

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
    KeyUpdateFailed = "Impossible to update encryption key",
    CannotUpdateLatest = "File is already at the latest encryption version"
}

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let input_file = crypted_file!(input_path.to_path_buf())?;

    let key = match nth_encription_key(input_file.encryption_version as usize) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::RetrievingKey),
    };

    let mut output_file = crypted_file!(output_path.to_path_buf())?;
    output_file.increment_layer()?;
    
    println!("efef");
    let encrypted: Vec<u8> = encrypt(&input_file.main_file_content()?, &key);
    output_file.update_file(encrypted)?;

    Ok(())
}

pub fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let input_file = crypted_file!(input_path.to_path_buf())?;

    if input_file.encryption_layer == 0 {
        return Err(EncryptionError::DecryptNotCryptedFile);
    }

    let key = match nth_encription_key(input_file.encryption_version as usize) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::RetrievingKey),
    };

    let mut output_file = crypted_file!(output_path.to_path_buf())?;
    output_file.decrement_layer()?;

    let decrypted_content: Vec<u8> = decrypt(&input_file.main_file_content()?, &key)?;

    output_file.update_file(decrypted_content)?;

    Ok(())
}

pub fn encrypt_file_x(
    input_path: &Path,
    output_path: &Path,
    times: u8,
) -> Result<ProgressBar, EncryptionError> {
    let mut counter = 0;
    let progress = arrow_progress(times as u64);
    progress.set_prefix("Encrypting file...");

    loop {
        encrypt_file(&input_path.to_path_buf(), &output_path.to_path_buf())?;
        progress.inc(1);
        counter += 1;

        if counter == times {
            progress.finish_and_clear();
            break;
        }
    }

    progress.finish_with_message("File encrypted!");
    Ok(progress)
}

pub fn decrypt_file_entirely(
    input_path: &Path,
    output_path: &Path,
) -> Result<ProgressBar, EncryptionError> {
    let file = crypted_file!(input_path.to_path_buf())?;
    let times = file.encryption_layer;

    let mut counter = 0;
    let progress = arrow_progress(times as u64);
    progress.set_prefix("Decrypting file...");

    loop {
        decrypt_file(&input_path.to_path_buf(), &output_path.to_path_buf())?;
        progress.inc(1);
        counter += 1;

        if counter == times {
            break;
        }
    }

    progress.finish_with_message("File decrypted!");
    Ok(progress)
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

    let new_line = config_line!(
        ENCRYPTION_KEYWORD,
        version,
        vec_as_string(generate_encryption_key(32))
    );

    config!()?.set_key(new_line)?;

    Ok(())
}

fn encryption_keys() -> Result<Vec<Vec<u8>>, EncryptionError> {
    let mut filtered_lines = config!()?.filter_map_lines(|line| -> Option<(u32, Vec<u8>)> {
        if line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)) {
            return Some(parse_encryption_key_line(line));
        }
        None
    })?;

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
    match config!()?.key_exists(ENCRYPTION_KEYWORD) {
        Ok(val) => {
            if let false = val {
                set_encryption_key()?
            }
            Ok(())
        }
        Err(_) => Err(EncryptionError::ConfigNotFound),
    }
}

pub fn init_new_encryption_key() -> Result<(), EncryptionError> {
    match config!()?.key_exists(ENCRYPTION_KEYWORD) {
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

pub fn update_file_encryption_key(filepath: PathBuf) -> Result<(), EncryptionError> {
    //update file key to latest
    let file = crypted_file!(filepath.clone())?;
    let initial_layer = file.encryption_layer;

    if file.encryption_version == latest_encryption_version()? {
        return Err(EncryptionError::CannotUpdateLatest);
    }

    // first we decrypt the file
    let progress_decrypt: ProgressBar = decrypt_file_entirely(&filepath, &filepath)?;
    thread::sleep(Duration::from_millis(250));
    progress_decrypt.finish_and_clear();

    // we encrypt it again with newly generated key
    let progress_encrypt: ProgressBar = encrypt_file_x(&filepath, &filepath, initial_layer as u8)?;

    thread::sleep(Duration::from_millis(250));
    progress_encrypt.finish_and_clear();

    Ok(())
}

pub fn purge_encryption_keys() -> Result<(), EncryptionError> {
    let mut config = config!()?;

    let filtered_content = config
        .filter_map_lines(|line| {
            if !line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)) {
                return Some(line.to_string());
            }
            None
        })?
        .join("\n");

    config.file.write_all(filtered_content.as_bytes())?;
    Ok(())
}

fn nth_encription_key(index: usize) -> Result<Vec<u8>, EncryptionError> {
    let keys = encryption_keys()?;
    let key = keys[index].clone();

    Ok(key)
}

pub fn latest_encryption_version() -> Result<u32, EncryptionError> {
    let mut filtered_lines: Vec<u32> = config!()?.filter_map_lines(|line| {
        if line.starts_with(&format!("{}=", ENCRYPTION_KEYWORD)) {
            return Some(
                parse_config_line!(line)
                    .unwrap()
                    .into_iter()
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            );
        }
        None
    })?;

    if filtered_lines.len() == 0 {
        return Err(EncryptionError::NoVersionFound);
    }

    filtered_lines.sort_by(|a, b| b.cmp(a));

    Ok(filtered_lines[0])
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
