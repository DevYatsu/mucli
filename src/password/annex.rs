const PASSWORD_KEYWORD: &str = "MUCLI_PASSWORD";
const PASSWORD_KEY_KEYWORD: &str = "MUCLI_KEY_PASSWORD";
const QUESTION_KEYWORD: &str = "MUCLI_QUESTION";

use std::io::{Error, Write};
use std::num::ParseIntError;

use simplecrypt::{decrypt, encrypt, DecryptionError};

use crate::encryption::EncryptionError;
use crate::utils::config_interact::vec_as_string;
use crate::utils::{
    config_interact::{string_as_vec, Config},
    GenericError,
};
use crate::utils::{generate_encryption_key, get_config_path};
use crate::{config, config_line, file_truncate};

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub PasswordError
    Io{source: Error} = "{source}",
    Format{source: ParseIntError} = "{source}",
    Generic{source: GenericError} = "{source}",
    Decrypt{source: DecryptionError} = "{source}",
    Encryption{source: EncryptionError} = "{source}",
    RetrievePassword = "Cannot retrieve password",
    RetrievePassowrdKey = "Cannot retrieve password encryption key",
    SetPassword = "Cannot set password in config.txt",
    EncryptPassword = "Failed to encrypt password",
    DecryptPassword = "Failed to decrypt password"
}

pub fn set_password(password: &str) -> Result<(), PasswordError> {
    let line = config_line!(PASSWORD_KEYWORD, vec_as_string(encrypt_password(password)?));
    config!()?.replace_key(PASSWORD_KEYWORD, line)?;
    Ok(())
}

pub fn get_password() -> Result<String, PasswordError> {
    let config = config!()?;
    if !config.key_exists(PASSWORD_KEYWORD)? {
        return Err(GenericError::KeyNotFound {
            key: PASSWORD_KEYWORD.to_owned(),
        }
        .into());
    }

    if let Some(password_string) = config.get_key(PASSWORD_KEYWORD)? {
        let crypted_password: Vec<u8> = string_as_vec::<u8>(&password_string)?;
        return decrypt_password(crypted_password);
    }

    Err(PasswordError::RetrievePassword)
}

pub fn encrypt_password(password: &str) -> Result<Vec<u8>, PasswordError> {
    encrypt_decrypt_password(password.as_bytes().to_vec(), true)
}

pub fn decrypt_password(crypted_value: Vec<u8>) -> Result<String, PasswordError> {
    let decrypted = encrypt_decrypt_password(crypted_value, false)?;
    Ok(String::from_utf8(decrypted).map_err(|_| PasswordError::DecryptPassword)?)
}

pub fn init_password_key() -> Result<(), PasswordError> {
    if !config!()?.key_exists(PASSWORD_KEY_KEYWORD)? {
        let new_line = config_line!(
            PASSWORD_KEY_KEYWORD,
            vec_as_string(generate_encryption_key(32))
        );
        config!()?.set_key(new_line)?
    }
    Ok(())
}

pub fn add_password_recovery_question(question: &str, answer: &str) -> Result<(), PasswordError> {
    let line: String = config_line!(QUESTION_KEYWORD, question, answer);
    config!()?.set_key(line)?;
    Ok(())
}
pub fn remove_password_recovery_question(index: usize) -> Result<(), PasswordError> {
    let questions: Vec<String> = retrieve_questions()?;

    let formatted_str = format!("{}=", config_line!(QUESTION_KEYWORD, questions[index]));

    let lines: Vec<String> = config!()?.filter_map_lines(|q| {
        if !q.starts_with(&formatted_str) {
            Some(q.to_string())
        } else {
            None
        }
    })?;
    let (mut file, _) = file_truncate!(get_config_path()?);

    file.write_all(lines.join("\n").as_bytes())?;

    Ok(())
}
pub fn retrieve_questions() -> Result<Vec<String>, PasswordError> {
    let config: Config = config!()?;
    if !config.key_exists(QUESTION_KEYWORD)? {
        return Err(GenericError::KeyNotFound {
            key: QUESTION_KEYWORD.to_owned(),
        }
        .into());
    }

    Ok(config.get_keys(QUESTION_KEYWORD))
}

fn encrypt_decrypt_password(
    password: Vec<u8>,
    encrypt_bool: bool,
) -> Result<Vec<u8>, PasswordError> {
    let config = config!()?;
    if let Some(password_string) = config.get_key(PASSWORD_KEY_KEYWORD)? {
        let key: Vec<u8> = string_as_vec::<u8>(&password_string)?;

        if encrypt_bool {
            return Ok(encrypt(&password, &key));
        } else {
            let decrypted = decrypt(&password, &key)?;
            return Ok(decrypted);
        }
    }

    Err(PasswordError::EncryptPassword)
}
