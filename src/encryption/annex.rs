use crate::crypted_file;
use crate::utils::config_interact::Config;
use crate::utils::file::CryptedFile;
use crate::utils::generate_encryption_key;
use crate::utils::line::Line;
use indicatif::ProgressBar;
use simplecrypt::{decrypt, encrypt};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";
use crate::utils::terminal::arrow_progress;

use super::{latest_encryption_version, EncryptionError};

pub fn encrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let mut input_file = crypted_file!(input_path.to_path_buf())?;

    let key = nth_encription_key(input_file.encryption_version()? as usize)?;

    let encrypted: Vec<u8> = encrypt(&input_file.main_file_content()?, &key);

    let mut output_file = crypted_file!(output_path.to_path_buf())?.from(&mut input_file)?;
    output_file.increment_layer()?;

    output_file.update_content(encrypted)?;

    Ok(())
}

pub fn decrypt_file(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), EncryptionError> {
    let mut input_file = crypted_file!(input_path.to_path_buf())?;

    if input_file.encryption_layer()? == 0 {
        return Err(EncryptionError::DecryptNotCryptedFile);
    }

    let key = nth_encription_key(input_file.encryption_version()? as usize)?;

    let decrypted_content: Vec<u8> = decrypt(&input_file.main_file_content()?, &key)?;

    let mut output_file = crypted_file!(output_path.to_path_buf())?.from(&mut input_file)?;
    output_file.decrement_layer()?;
    output_file.update_content(decrypted_content)?;

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
    let times = file.encryption_layer()?;

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

pub fn init_encryption_key() -> Result<(), EncryptionError> {
    match Config::open()?.key_exists(ENCRYPTION_KEYWORD) {
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
    match Config::open()?.key_exists(ENCRYPTION_KEYWORD) {
        Ok(val) => {
            if let true = val {
                set_encryption_key()?
            }
            Ok(())
        }
        Err(_) => Err(EncryptionError::ConfigNotFound),
    }
}

pub fn update_file_encryption_key(filepath: &PathBuf) -> Result<(), EncryptionError> {
    //update file key to latest
    let file = crypted_file!(filepath.to_path_buf())?;
    let initial_layer = file.encryption_layer()?;

    if file.encryption_version()? == latest_encryption_version()? {
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
    let mut config = Config::open()?;

    let new_line = if let Some(line) = config.get_line(ENCRYPTION_KEYWORD) {
        let mut parsed_line: Line<Vec<Vec<u8>>> = Line::from(&line)?;
        parsed_line.set_new(vec![]);
        parsed_line
    } else {
        Line::new(ENCRYPTION_KEYWORD, vec![])
    };

    config.replace_key(new_line)?;
    Ok(())
}

fn set_encryption_key() -> Result<(), EncryptionError> {
    let mut config = Config::open()?;
    let new_line = if let Some(line) = config.get_line(ENCRYPTION_KEYWORD) {
        let mut parsed_line: Line<Vec<Vec<u8>>> = Line::from(&line)?;
        parsed_line.add(generate_encryption_key(32));
        parsed_line
    } else {
        Line::new(ENCRYPTION_KEYWORD, vec![generate_encryption_key(32)])
    };

    config.set_line(new_line)?;

    Ok(())
}

fn retrieve_encryption_keys() -> Result<Vec<Vec<u8>>, EncryptionError> {
    let config = Config::open()?;
    let encryption_keys: Line<Vec<Vec<u8>>> =
        if let Some(line) = config.get_line(ENCRYPTION_KEYWORD) {
            Line::from(&line)?
        } else {
            return Err(EncryptionError::KeyNotExist);
        };

    if encryption_keys.value.is_empty() {
        return Err(EncryptionError::NoKeyFound);
    }

    Ok(encryption_keys.value)
}

fn nth_encription_key(index: usize) -> Result<Vec<u8>, EncryptionError> {
    let keys = retrieve_encryption_keys()?;
    if let Some(key) = keys.get(index) {
        Ok(key.clone())
    } else {
        Err(EncryptionError::KeyNotExist)
    }
}
