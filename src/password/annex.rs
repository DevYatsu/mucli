const PASSWORD_KEYWORD: &str = "MUCLI_PASSWORD";
const PASSWORD_KEY_KEYWORD: &str = "MUCLI_KEY_PASSWORD";
const QUESTION_KEYWORD: &str = "MUCLI_QUESTION";

use std::io::Error;
use std::num::ParseIntError;

use simplecrypt::{decrypt, encrypt, DecryptionError};

use crate::encryption::EncryptionError;
use crate::utils::generate_encryption_key;
use crate::utils::line::{Line, LineError};
use crate::utils::{config_interact::Config, GenericError};

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
    Config::open()?.replace_key(Line::new(PASSWORD_KEYWORD, encrypt_password(password)?))?;
    Ok(())
}

pub fn get_password() -> Result<String, PasswordError> {
    let config = Config::open()?;

    if let Some(password_line) = config.get_line(PASSWORD_KEYWORD) {
        let password: Line<Vec<u8>> = Line::from(&password_line)?;
        Ok(decrypt_password(password.value)?)
    } else {
        Err(PasswordError::RetrievePassword)
    }
}

pub fn encrypt_password(password: &str) -> Result<Vec<u8>, PasswordError> {
    encrypt_decrypt_password(password.as_bytes().to_vec(), true)
}

pub fn decrypt_password(crypted_value: Vec<u8>) -> Result<String, PasswordError> {
    let decrypted = encrypt_decrypt_password(crypted_value, false)?;
    Ok(String::from_utf8(decrypted).map_err(|_| PasswordError::DecryptPassword)?)
}

pub fn init_password_key() -> Result<(), PasswordError> {
    let mut config = Config::open()?;
    if let None = config.get_line(PASSWORD_KEY_KEYWORD) {
        config.set_line(Line::new(PASSWORD_KEY_KEYWORD, generate_encryption_key(32)))?
    }
    Ok(())
}

pub fn add_password_recovery_question(question: &str, answer: &str) -> Result<(), PasswordError> {
    let config = Config::open()?;

    let line = if let Some(line) = config.get_line(QUESTION_KEYWORD) {
        let mut parsed_line: Line<Vec<(String, String)>> = Line::from(&line)?;
        parsed_line.add((question.trim().to_string(), answer.trim().to_string()));
        parsed_line
    } else {
        Line::new(
            QUESTION_KEYWORD,
            vec![(question.trim().to_string(), answer.trim().to_string())],
        )
    };
    Config::open()?.replace_key(line)?;
    Ok(())
}
pub fn remove_password_recovery_question(index: usize) -> Result<(), PasswordError> {
    let mut questions = retrieve_questions()?;

    questions.value.remove(index);

    Config::open()?.replace_key(questions)?;
    Ok(())
}
pub fn retrieve_questions() -> Result<Line<Vec<(String, String)>>, PasswordError> {
    let config: Config = Config::open()?;
    if let Some(line) = config.get_line(QUESTION_KEYWORD) {
        Ok(Line::from(&line)?)
    } else {
        return Err(GenericError::KeyNotFound {
            key: QUESTION_KEYWORD.to_owned(),
        }
        .into());
    }
}

fn encrypt_decrypt_password(
    password: Vec<u8>,
    encrypt_bool: bool,
) -> Result<Vec<u8>, PasswordError> {
    let config = Config::open()?;
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
