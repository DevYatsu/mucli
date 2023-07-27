mod annex;

use std::{
    env::current_dir,
    io::Error,
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use simplecrypt::DecryptionError;

use crate::{
    encryption::annex::{
        encrypt_file, encrypt_file_x, encrypted_file_path, init_encryption_key,
        init_new_encryption_key, purge_encryption_keys, update_file_encryption_key,
    },
    parse_config_line, print_err, print_solution, print_success,
    utils::{config_interact::Config, line::LineError, GenericError},
};
use custom_error::custom_error;

const ENCRYPTION_KEYWORD: &str = "MUCLI_ENCRYPT";

custom_error! {pub EncryptionError
    Io{source: Error} = "{source}",
    Generic{source: GenericError} = "{source}",
    Decrypt{source: DecryptionError} = "{source}",
    Line{source: LineError} = "{source}",
    NoKeyFound = "No key found in config.txt",
    RetrievingKey = "Error retrieving encryption key",
    KeyNotExist = "Encryption key does not exist",
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
    CannotUpdateLatest = "File is already at the latest encryption version",
    CannotProcessVoidFile = "Cannot process empty file"
}

use self::annex::{decrypt_file, decrypt_file_entirely, decrypted_file_path};

pub fn encrypt_command(sub_matches: &ArgMatches) {
    if let Err(_) = init_encryption_key() {
        // initialize encryption key if 1st time using command
        print_err!("Error initializing encryption key!");
        return;
    }
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        let file_path: &Path = Path::new(filepath);
        if file_path.exists() {
            if let true = sub_matches.get_flag("sfile") {
                if let Some(times) = sub_matches.get_one::<u8>("times") {
                    match encrypt_file_x(file_path, file_path, *times) {
                        Ok(pb) => {
                            pb.finish_and_clear();
                            print_success!(
                                "{:?} content was encrypted {} times successfully",
                                &file_path,
                                times
                            );
                        }
                        Err(e) => print_err!("(encryption failed): {}", e),
                    };
                } else {
                    match encrypt_file(&file_path.to_path_buf(), &file_path.to_path_buf()) {
                        Ok(_) => {
                            print_success!("{:?} content replaced with crypted one!", &file_path)
                        }
                        Err(e) => print_err!("(encryption failed): {}", e),
                    };
                }
            } else if let true = sub_matches.get_flag("cdir") {
                match current_dir() {
                    Ok(current_dir) => {
                        let output_path = encrypted_file_path(&file_path, &current_dir);
                        if let Some(times) = sub_matches.get_one::<u8>("times") {
                            match encrypt_file_x(&file_path, &output_path, *times) {
                                Ok(pb) => {
                                    pb.finish_and_clear();
                                    print_success!(
                                        "File encrypted {} times and saved as {:?}",
                                        times,
                                        output_path
                                    );
                                }
                                Err(e) => print_err!("(encryption failed): {}", e),
                            };
                        } else {
                            match encrypt_file(&file_path.to_path_buf(), &output_path.to_path_buf())
                            {
                                Ok(_) => {
                                    print_success!("Encrypted file saved as {:?}!", output_path)
                                }
                                Err(e) => print_err!("(encryption failed): {}", e),
                            };
                        }
                    }
                    Err(error) => {
                        print_err!("Failed to get current directory: {}", error)
                    }
                }
            } else if let true = sub_matches.get_flag("ukey") {
                match update_file_encryption_key(&file_path.to_path_buf()) {
                    Ok(_) => print_success!(
                        "{} updated without issue",
                        file_path.file_name().unwrap().to_string_lossy().to_string()
                    ),
                    Err(EncryptionError::CannotUpdateLatest) => {
                        print_err!("{}", EncryptionError::CannotUpdateLatest.to_string())
                    }
                    Err(e) => print_err!("Key updating failed: {}", e),
                }
            } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                match Path::new(output_dir).is_dir() {
                    true => {
                        let output_path = encrypted_file_path(&file_path, &output_dir);
                        if let Some(times) = sub_matches.get_one::<u8>("times") {
                            match encrypt_file_x(&file_path, &output_path, *times) {
                                Ok(pb) => {
                                    pb.finish_and_clear();
                                    print_success!(
                                        "File encrypted {} times and saved as {:?}",
                                        times,
                                        output_path
                                    );
                                }
                                Err(e) => print_err!("(encryption failed) {}", e),
                            };
                        } else {
                            match encrypt_file(&file_path.to_path_buf(), &output_path.to_path_buf())
                            {
                                Ok(_) => {
                                    print_success!("Encrypted file saved as {:?}!", output_path)
                                }
                                Err(e) => print_err!("(encryption failed) {}", e),
                            };
                        }
                    }
                    false => print_err!("Failed to get {:?} directory", output_dir),
                }
            } else {
                match file_path.parent() {
                    Some(parent_dir) => {
                        let output_path = encrypted_file_path(&file_path, &parent_dir);
                        if let Some(times) = sub_matches.get_one::<u8>("times") {
                            match encrypt_file_x(
                                &file_path.to_path_buf(),
                                &output_path.to_path_buf(),
                                *times,
                            ) {
                                Ok(pb) => {
                                    pb.finish_and_clear();
                                    print_success!(
                                        "{:?} content was encrypted {} times successfully",
                                        &file_path,
                                        times
                                    );
                                }
                                Err(e) => print_err!("(encryption failed): {}", e),
                            };
                        } else {
                            match encrypt_file(&file_path.to_path_buf(), &output_path.to_path_buf())
                            {
                                Ok(_) => {
                                    print_success!("Encrypted file saved as {:?}!", output_path)
                                }
                                Err(e) => print_err!("(encryption failed) {}", e),
                            };
                        }
                    }
                    None => print_err!("Failed to get target file parent directory"),
                }
            }
        } else {
            print_err!("{:?} does not exist!", filepath);
            print_solution!("Check target file and try again");
            return;
        }
    } else if let true = sub_matches.get_flag("ukey") {
        if let Err(_) = init_new_encryption_key() {
            // initialize encryption key if 1st time using command
            print_err!("Error updating encryption key!");
            return;
        }
        print_success!("Encryption keys updated successfully")
    } else if let true = sub_matches.get_flag("purge") {
        if let Err(_) = purge_encryption_keys() {
            // initialize encryption key if 1st time using command
            print_err!("Error purging encryption keys!");
            return;
        }
        print_success!("Encryption keys purged successfully!")
    }
}

pub fn decrypt_command(sub_matches: &ArgMatches) {
    if let Err(_) = init_encryption_key() {
        // initialize encryption key if 1st time using command
        print_err!("Error initializing encryption key!");
        return;
    }
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        let file_path: &Path = Path::new(filepath);
        if file_path.exists() {
            if let true = sub_matches.get_flag("sfile") {
                if let true = sub_matches.get_flag("entirely") {
                    match decrypt_file_entirely(&file_path, &file_path) {
                        Ok(pb) => {
                            pb.finish_and_clear();
                            print_success!(
                                "{:?} content was entirely decrypted successfully",
                                &file_path
                            );
                        }
                        Err(e) => print_err!("(decryption failed): {}", e),
                    };
                } else {
                    match decrypt_file(&file_path.to_path_buf(), &file_path.to_path_buf()) {
                        Ok(_) => {
                            print_success!("{:?} content replaced with decrypted one!", &file_path)
                        }
                        Err(e) => print_err!("(decryption failed): {}", e),
                    };
                }
            } else if let true = sub_matches.get_flag("cdir") {
                match current_dir() {
                    Ok(current_dir) => {
                        let output_path = decrypted_file_path(&file_path, &current_dir);
                        if let true = sub_matches.get_flag("entirely") {
                            match decrypt_file_entirely(&file_path, &output_path) {
                                Ok(pb) => {
                                    pb.finish_and_clear();
                                    print_success!(
                                        "File was entirely decrypted  as {:?}",
                                        output_path
                                    );
                                }
                                Err(e) => print_err!("(decryption failed): {}", e),
                            };
                        } else {
                            match decrypt_file(&file_path.to_path_buf(), &output_path.to_path_buf())
                            {
                                Ok(_) => {
                                    print_success!("Decrypted file saved as {:?}!", output_path)
                                }
                                Err(e) => print_err!("(decryption failed): {}", e),
                            };
                        }
                    }
                    Err(error) => {
                        print_err!("Failed to get current directory: {}", error)
                    }
                }
            } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                match Path::new(output_dir).is_dir() {
                    true => {
                        let output_path = decrypted_file_path(&file_path, &output_dir);
                        if let true = sub_matches.get_flag("entirely") {
                            match decrypt_file_entirely(&file_path, &output_path) {
                                Ok(pb) => {
                                    pb.finish_and_clear();
                                    print_success!(
                                        "File was entirely decrypted  as {:?}",
                                        output_path
                                    );
                                }
                                Err(e) => print_err!("(decryption failed): {}", e),
                            };
                        } else {
                            match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                Ok(_) => {
                                    print_success!("Decrypted file saved as {:?}!", output_path)
                                }
                                Err(e) => print_err!("(decryption failed): {}", e),
                            };
                        }
                    }
                    false => print_err!("Failed to get {:?} directory", output_dir),
                }
            } else {
                match file_path.parent() {
                    Some(parent_dir) => {
                        let output_path = decrypted_file_path(&file_path, &parent_dir);
                        if let true = sub_matches.get_flag("entirely") {
                            match decrypt_file_entirely(&file_path, &output_path) {
                                Ok(pb) => {
                                    pb.finish_and_clear();
                                    print_success!(
                                        "File was entirely decrypted  as {:?}",
                                        output_path
                                    );
                                }
                                Err(e) => print_err!("(decryption failed): {}", e),
                            };
                        } else {
                            match decrypt_file(&file_path.to_path_buf(), &output_path.to_path_buf())
                            {
                                Ok(_) => {
                                    print_success!("Decrypted file saved as {:?}!", output_path)
                                }
                                Err(e) => print_err!("(decryption failed): {}", e),
                            };
                        }
                    }
                    None => print_err!("Failed to get target file parent directory"),
                }
            }
        } else {
            print_err!("{:?} does not exist!", filepath);
            print_solution!("Check target file and try again");
            return;
        }
    }
}

pub fn latest_encryption_version() -> Result<u32, EncryptionError> {
    let mut filtered_lines: Vec<u32> = Config::open()?.filter_map_lines(|line| {
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
