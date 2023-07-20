use std::fs::File;

const PASSWORD_KEYWORD: &str = "MUCLI_PASSWORD";
const CONFIG_FILE: &str = "config.txt";
const PASSWORD_KEY_KEYWORD: &str = "MUCLI_KEY_PASSWORD";

use std::io::{prelude::*, Error};

use simplecrypt::{decrypt, encrypt};

use crate::encryption::{generate_encryption_key, EncryptionError};
use crate::utils::{get_keys, key_exists, replace_key, set_key, string_as_key};

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub PasswordError
    Io{source: Error} = "{source}",
    RetrievePassword = "Cannot retrieve password",
    SetPassword = "Cannot set password in config.txt",
}

pub fn set_password(password: &str) -> Result<(), EncryptionError> {
    let line = format!("{}={:?}", PASSWORD_KEYWORD, encrypt_password(password)?);
    replace_key::<EncryptionError>(PASSWORD_KEYWORD, line)?;
    Ok(())
}

pub fn get_password() -> Result<String, EncryptionError> {
    if !key_exists::<EncryptionError>(PASSWORD_KEYWORD)? {
        return Err(EncryptionError::ConfigNotFound);
    }

    let mut file: File = File::open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    // takes the 0 index cause there should only be one password
    let lines: Vec<String> = get_keys::<EncryptionError>(PASSWORD_KEYWORD)?;

    let key: Vec<u8> = match string_as_key::<u8>(&lines[0]) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::EncryptPassword),
    };

    Ok(decrypt_password(key)?)
}

fn encrypt_password(password: &str) -> Result<Vec<u8>, EncryptionError> {
    let key = match string_as_key::<u8>(&get_keys::<Error>(PASSWORD_KEY_KEYWORD)?[0]) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::EncryptPassword),
    };

    let encrypted = encrypt(password.as_bytes(), &key);
    Ok(encrypted)
}
pub fn decrypt_password(crypted_value: Vec<u8>) -> Result<String, EncryptionError> {
    let key: Vec<u8> = match string_as_key::<u8>(&get_keys::<Error>(PASSWORD_KEY_KEYWORD)?[0]) {
        Ok(k) => k,
        Err(_) => return Err(EncryptionError::EncryptPassword),
    };

    let decrypted = match decrypt(&crypted_value, &key) {
        Ok(val) => val,
        Err(_) => return Err(EncryptionError::DecryptPassword),
    };

    Ok(String::from_utf8(decrypted).unwrap())
}

pub fn init_password_key() -> Result<(), Error> {
    if !key_exists::<Error>(PASSWORD_KEY_KEYWORD)? {
        let new_line = format!("{}={:?}", PASSWORD_KEY_KEYWORD, generate_encryption_key(32));
        set_key::<Error>(new_line)?
    }
    Ok(())
}
