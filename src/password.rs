const PASSWORD_KEYWORD: &str = "MUCLI_PASSWORD";
const CONFIG_FILE: &str = "config.txt";
const PASSWORD_KEY_KEYWORD: &str = "MUCLI_KEY_PASSWORD";

use std::io::Error;
use std::num::ParseIntError;

use simplecrypt::{decrypt, encrypt, DecryptionError};

use crate::encryption::{generate_encryption_key, EncryptionError};
use crate::utils::{
    config_interact::{get_key, key_exists, replace_key, set_key, string_as_key},
    GenericError,
};

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub PasswordError
    Io{source: Error} = "{source}",
    Format{source: ParseIntError} = "{source}",
    Generic{source: GenericError} = "{source}",
    Decrypt{source: DecryptionError} = "{source}",
    Encryption{source: EncryptionError} = "{source}",
    RetrievePassword = "Cannot retrieve password",
    SetPassword = "Cannot set password in config.txt",
    EncryptPassword = "Failed to encrypt password",
    DecryptPassword = "Failed to decrypt password"
}

pub fn set_password(password: &str) -> Result<(), PasswordError> {
    let line = format!("{}={:?}", PASSWORD_KEYWORD, encrypt_password(password)?);
    replace_key(PASSWORD_KEYWORD, line)?;
    Ok(())
}

pub fn get_password() -> Result<String, PasswordError> {
    if !key_exists(PASSWORD_KEYWORD)? {
        return Err(GenericError::KeyNotFound {
            key: PASSWORD_KEYWORD.to_owned(),
            filename: CONFIG_FILE.to_owned(),
        }
        .into());
    }

    let crypted_password: Vec<u8> = string_as_key::<u8>(&get_key(PASSWORD_KEYWORD)?)?;

    Ok(decrypt_password(crypted_password)?)
}

fn encrypt_password(password: &str) -> Result<Vec<u8>, PasswordError> {
    let key = string_as_key::<u8>(&get_key(PASSWORD_KEY_KEYWORD)?)?;

    let encrypted = encrypt(password.as_bytes(), &key);
    Ok(encrypted)
}
pub fn decrypt_password(crypted_value: Vec<u8>) -> Result<String, PasswordError> {
    let key: Vec<u8> = string_as_key::<u8>(&get_key(PASSWORD_KEY_KEYWORD)?)?;

    let decrypted = decrypt(&crypted_value, &key)?;

    Ok(String::from_utf8(decrypted).unwrap())
}

pub fn init_password_key() -> Result<(), PasswordError> {
    if !key_exists(PASSWORD_KEY_KEYWORD)? {
        let new_line = format!("{}={:?}", PASSWORD_KEY_KEYWORD, generate_encryption_key(32));
        set_key(new_line)?
    }
    Ok(())
}
