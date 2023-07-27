const PASSWORD_KEYWORD: &str = "MUCLI_PASSWORD";
const PASSWORD_KEY_KEYWORD: &str = "MUCLI_KEY_PASSWORD";
const QUESTION_KEYWORD: &str = "MUCLI_QUESTION";

use std::io::{Error, Write};
use std::num::ParseIntError;

use simplecrypt::{decrypt, encrypt, DecryptionError};

use crate::encryption::EncryptionError;
use crate::utils::line::{Line, LineError};
use crate::utils::{config_interact::Config, GenericError};
use crate::utils::{generate_encryption_key, get_config_path};
use crate::{config, config_line, file_truncate};

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub PasswordError
    Io{source: Error} = "{source}",
    Line{source: LineError} = "{source}",
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
    config!()?.replace_key(Line::new(PASSWORD_KEYWORD, encrypt_password(password)?))?;
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

    if let Some(password_line) = config.get_line(PASSWORD_KEYWORD) {
        let password: Line<Vec<u8>> = Line::from(&password_line)?;
        return Ok(decrypt_password(password.value)?);
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
    let mut config = config!()?;
    if let None = config.get_line(PASSWORD_KEY_KEYWORD) {
        config.set_line(Line::new(PASSWORD_KEY_KEYWORD, generate_encryption_key(32)))?
    }
    Ok(())
}

pub fn add_password_recovery_question(question: &str, answer: &str) -> Result<(), PasswordError> {
    let config = config!()?;
    let line = if let Some(line) = config.get_line(QUESTION_KEYWORD) {
        let mut parsed_line: Line<Vec<(String, String)>> = Line::from(&line)?;
        parsed_line.add((question.to_string(), answer.to_string()));
        parsed_line
    } else {
        Line::new(
            QUESTION_KEYWORD,
            vec![(question.to_string(), answer.to_string())],
        )
    };
    config!()?.replace_key(line)?;
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
    if let Some(password_line) = config.get_line(PASSWORD_KEY_KEYWORD) {
        let line: Line<Vec<u8>> = Line::from(&password_line)?;

        if encrypt_bool {
            return Ok(encrypt(&password, &line.value));
        } else {
            return Ok(decrypt(&password, &line.value)?);
        }
    }

    Err(PasswordError::EncryptPassword)
}
