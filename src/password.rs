// update questions to store them as crypted content
use crate::encryption::EncryptionError;
use crate::utils::generate_encryption_key;
use crate::utils::line::{Line, LineError};
use crate::utils::{config_interact::Config, GenericError};
use crate::{print_err, print_solution, print_success};

use clap::ArgMatches;
use custom_error::custom_error;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use simplecrypt::{decrypt, encrypt, DecryptionError};
use std::io::Error;
use std::num::ParseIntError;

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

const PASSWORD_KEYWORD: &str = "MUCLI_PASSWORD";
const PASSWORD_KEY_KEYWORD: &str = "MUCLI_KEY_PASSWORD";
const QUESTION_KEYWORD: &str = "MUCLI_QUESTION";

pub fn password_command(sub_matches: &ArgMatches) {
    if let Err(_) = init_password_key() {
        // initialize encryption key if 1st time using command
        print_err!("Initializing password key failed!");
        return;
    }

    if let true = sub_matches.contains_id("init") {
        match get_password() {
            Ok(_) => {
                print_err!("Password is already set");
                print_solution!("Use \"password --change\" to modify it");
            }
            Err(_) => {
                let password: String =
                    if let Some(new_password) = sub_matches.get_one::<String>("init") {
                        Password::with_theme(&ColorfulTheme::default())
                            .with_prompt("Confirm your password")
                            .validate_with(|input: &String| -> Result<(), &str> {
                                if input != new_password {
                                    return Err("Passwords don't match");
                                }
                                Ok(())
                            })
                            .interact()
                            .unwrap()
                    } else {
                        Password::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter your password")
                            .with_confirmation("Confirm your password", "The passwords don't match")
                            .interact()
                            .unwrap()
                    };

                match set_password(&password) {
                    Ok(_) => {
                        print_success!("\"{}\" was successfully set as your new password", password)
                    }
                    Err(e) => print_err!("Failed to set password! > {}", e),
                };
            }
        }
    } else if let true = sub_matches.contains_id("change") {
        match get_password() {
            Ok(password) => {
                if let Some(password_input) = sub_matches.get_one::<String>("change") {
                    if *password_input != password {
                        print_err!("Invalid password put as argument!");
                        return;
                    }
                } else {
                    let actual_password: String = Password::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter your current password")
                        .interact()
                        .unwrap();
                    if actual_password != password {
                        print_err!("Wrong password!");
                        return;
                    }
                }

                let new_password: String = Password::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your new password")
                    .with_confirmation("Confirm your new password", "Passwords don't match")
                    .interact()
                    .unwrap();

                match set_password(&new_password) {
                    Ok(_) => {
                        print_success!(
                            "\"{}\" was successfully set as your new password",
                            new_password
                        );
                        return;
                    }
                    Err(e) => {
                        print_err!("{}", e);
                        return;
                    }
                };
            }
            Err(_) => {
                print_err!("Password has not been set yet");
                print_solution!("Use \"password --init\" to set it");
                return;
            }
        }
    } else if let true = sub_matches.contains_id("modifyQ") {
        match get_password() {
            Ok(password) => {
                if let Some(password_input) = sub_matches.get_one::<String>("modifyQ") {
                    if *password_input != password {
                        print_err!("Wrong password!");
                        return;
                    }
                } else {
                    let actual_password: String = Password::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter your current password")
                        .interact()
                        .unwrap();
                    if actual_password != password {
                        print_err!("Wrong password!");
                        return;
                    }
                }
                // password is right from here on

                loop {
                    let items = vec![
                        "List questions",
                        "Add a question",
                        "Remove a question",
                        "Quit",
                    ];
                    let chosen: usize = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Choose an option")
                        .items(&items)
                        .default(0)
                        .interact()
                        .unwrap();

                    if chosen == 0 {
                        match retrieve_questions() {
                            Ok(questions) => {
                                println!("Questions List:");
                                for (question, _) in &questions.value {
                                    println!("• {}", question);
                                }
                            }
                            Err(_) => print_err!("No question set"),
                        }
                    } else if chosen == 1 {
                        let question: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Your Question")
                            .validate_with(|input: &String| -> Result<(), &str> {
                                let questions = match retrieve_questions() {
                                    Ok(questions) => questions.value,
                                    Err(_) => vec![],
                                };
                                if questions.iter().any(|(q, _)| q == &format!("{}", input)) {
                                    Err("Cannot set a question twice")
                                } else if input.len() > 9 {
                                    Ok(())
                                } else {
                                    Err("Question must be at least 10 characters long")
                                }
                            })
                            .interact_text()
                            .unwrap();

                        let answer: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("The Answer")
                            .validate_with(|input: &String| -> Result<(), &str> {
                                if input.len() > 1 {
                                    Ok(())
                                } else {
                                    Err("Question must be at least 3 characters long")
                                }
                            })
                            .interact_text()
                            .unwrap();

                        match add_password_recovery_question(&question, &answer) {
                            Ok(_) => {
                                print_success!("Question and answer add successfully!")
                            }
                            Err(_) => print_err!("Failed to add question and answer"),
                        };
                    } else if chosen == 2 {
                        let mut choices: Vec<String> = match retrieve_questions() {
                            Ok(questions) => questions.value.into_iter().map(|(q, _)| q).collect(),
                            Err(_) => {
                                print_err!("No question set");
                                continue;
                            }
                        };
                        choices.push("Cancel".to_string());

                        let chosen: usize = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Which Question to remove?")
                            .items(&choices)
                            .default(0)
                            .interact()
                            .unwrap();

                        if chosen == choices.len() - 1 {
                            continue;
                        }
                        match remove_password_recovery_question(chosen) {
                            Ok(_) => print_success!("Question removed successfully!"),
                            Err(_) => print_err!("Failed to remove question"),
                        };
                    } else if chosen == 3 {
                        break;
                    }
                }
            }
            Err(_) => {
                print_err!("Password has not been set yet");
                print_solution!("Use \"password --init\" to set it");
                return;
            }
        }
    } else if let true = sub_matches.contains_id("reset") {
        match get_password() {
            Ok(_) => match retrieve_questions() {
                Ok(question_line) => {
                    let mut questions: Vec<String> = question_line
                        .value
                        .iter()
                        .map(|(q, _)| q.to_string())
                        .collect();
                    let answers: Vec<String> = question_line
                        .value
                        .iter()
                        .map(|(_, a)| a.to_string())
                        .collect();
                    if questions.len() < 3 {
                        print_err!("Not enough questions were set");
                        print_solution!("3 questions are needed to reset password");
                        return;
                    }
                    questions.push("Cancel".to_string());
                    let mut answered_questions: Vec<String> = vec![];

                    loop {
                        let chosen: usize = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("A question to answer")
                            .items(&questions)
                            .default(0)
                            .interact()
                            .unwrap();

                        if chosen == questions.len() - 1 {
                            return;
                        }

                        let answer: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Your Answer")
                            .interact_text()
                            .unwrap();

                        questions = if answer == answers[chosen + answered_questions.len()] {
                            questions.remove(chosen);
                            answered_questions.push(answer);
                            questions
                        } else {
                            questions
                                .into_iter()
                                .filter(|s| !answered_questions.contains(s))
                                .collect()
                        };

                        if answered_questions.len() == 3 {
                            print_success!("That's 3 right answers! Change your password now!");
                            break;
                        }
                    }
                    let new_password: String = Password::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter your new password")
                        .with_confirmation("Confirm your new password", "The passwords don't match")
                        .interact()
                        .unwrap();

                    match set_password(&new_password) {
                        Ok(_) => {
                            print_success!("New password \"{}\" set successfully!", new_password)
                        }
                        Err(_) => print_err!("Failed to set new password"),
                    };
                }
                Err(_) => {
                    print_err!("No password recovery question set");
                    print_solution!("Use \"password -m\" to set questions");
                    return;
                }
            },
            Err(_) => {
                print_err!("Password has not been set yet");
                print_solution!("Use \"password --init\" to set it");
                return;
            }
        }
    }
}

fn set_password(password: &str) -> Result<(), PasswordError> {
    Config::open()?.replace_key(Line::new(PASSWORD_KEYWORD, encrypt_password(password)?))?;
    Ok(())
}

fn get_password() -> Result<String, PasswordError> {
    let config = Config::open()?;

    if let Some(password_line) = config.get_line(PASSWORD_KEYWORD) {
        let password: Line<Vec<u8>> = Line::from(&password_line)?;
        Ok(decrypt_password(password.value)?)
    } else {
        Err(PasswordError::RetrievePassword)
    }
}

fn encrypt_password(password: &str) -> Result<Vec<u8>, PasswordError> {
    encrypt_decrypt_password(password.as_bytes().to_vec(), true)
}

fn decrypt_password(crypted_value: Vec<u8>) -> Result<String, PasswordError> {
    let decrypted = encrypt_decrypt_password(crypted_value, false)?;
    Ok(String::from_utf8(decrypted).map_err(|_| PasswordError::DecryptPassword)?)
}

fn init_password_key() -> Result<(), PasswordError> {
    let mut config = Config::open()?;
    if let None = config.get_line(PASSWORD_KEY_KEYWORD) {
        config.set_line(Line::new(PASSWORD_KEY_KEYWORD, generate_encryption_key(32)))?
    }
    Ok(())
}

fn add_password_recovery_question(question: &str, answer: &str) -> Result<(), PasswordError> {
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
fn remove_password_recovery_question(index: usize) -> Result<(), PasswordError> {
    let mut questions = retrieve_questions()?;

    questions.value.remove(index);

    Config::open()?.replace_key(questions)?;
    Ok(())
}
fn retrieve_questions() -> Result<Line<Vec<(String, String)>>, PasswordError> {
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
