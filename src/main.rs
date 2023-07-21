mod encryption;
mod password;
mod utils;

use crate::{
    encryption::{decrypt_file, encrypt_file, encrypt_file_x},
    password::add_password_recovery_question,
};
use clap::{arg, command, ArgAction, ArgGroup, Command};
use dialoguer::{
    theme::{self, ColorfulTheme},
    Input, Password, Select,
};
use encryption::{
    decrypt_file_entirely, decrypted_file_path, encrypted_file_path, init_encryption_key,
    update_encryption_key, update_file_encryption_key, EncryptionError,
};
use password::{
    get_password, init_password_key, remove_password_recovery_question, retrieve_questions,
    set_password,
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};
use utils::config_interact::filter_map_lines;

fn main() {
    let matches = command!()
        .author("yatsu")
        .name("mucli")
        .version("0.1.0")
        .about("A multi-purposes client line interface: mucli!")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("password")
                .about("Set a security password to access sensible informations")
                .group(
                    ArgGroup::new("password_action")
                        .required(true)
                        .args(["init", "change", "reset", "modifyQ"]),
                )
                .arg(arg!(-'i' --"init" [NEW_PASSWORD] "Set password for first time").action(ArgAction::Set))
                .arg(arg!(-'c' --"change" [ACTUAL_PASSWORD] "Change password when set").action(ArgAction::Set))
                .arg(arg!(-'r' --"reset" "Reset password by answering a set of questions").action(ArgAction::SetTrue))
                .arg(arg!(-'m' --"modifyQ" [PASSWORD] "Add and remove questions you will have to answer in order to reset your password").action(ArgAction::Set))
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts the specified file and place the output file in specified dir")
                .group(
                    ArgGroup::new("encrypt_actions")
                        .required(false)
                        .args(["ukey", "cdir", "sfile"]),
                )
                .arg(arg!(-'u' --"ukey" "Update encryption key or update encryption key of a file to the latest version").action(ArgAction::SetTrue))
                .arg(arg!(-'c' --"cdir" "Place output file in current dir").action(ArgAction::SetTrue))
                .arg(arg!(-'s' --"sfile" "Select target file as output file").action(ArgAction::SetTrue))
                .arg(arg!(-'p' --"purge" "Get rid of all the encryption keys to start anew").action(ArgAction::SetTrue))
                .arg(arg!(-'t' --"times" <TIMES> "Encrypt x times the file").action(ArgAction::Set).value_parser(clap::value_parser!(u8)))
                .arg(arg!([FILEPATH] "file path of the target file").required_unless_present_all(["ukey"]).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([OUTPUTDIR] "output directory [defaults: file dir]").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts the specified file and place the output file in specified dir")
                .group(
                    ArgGroup::new("decrypt_actions")
                        .required(false)
                        .args(["cdir", "sfile"]),
                )
                .arg(arg!(-'c' --"cdir" "Place output file in current dir").action(ArgAction::SetTrue))
                .arg(arg!(-'s' --"sfile" "Select target file as output file").action(ArgAction::SetTrue))
                .arg(arg!(-'e' --"entirely" "Entirely decrypt target file").action(ArgAction::SetTrue))
                .arg(arg!([FILEPATH] "file path of the target file").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([OUTPUTDIR] "output directory [defaults: file dir]").value_parser(clap::value_parser!(PathBuf))),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
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
                                Err(_) => print_err!("Failed to encrypt file"),
                            };
                        } else {
                            match encrypt_file(&file_path.to_path_buf(), &file_path.to_path_buf()) {
                                Ok(_) => {
                                    print_success!(
                                        "{:?} content replaced with crypted one!",
                                        &file_path
                                    )
                                }
                                Err(_) => print_err!("Failed to encrypt file"),
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
                                        Err(_) => print_err!("Failed to encrypt file"),
                                    };
                                } else {
                                    match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                        Ok(_) => {
                                            print_success!(
                                                "Encrypted file saved as {:?}!",
                                                output_path
                                            )
                                        }
                                        Err(_) => print_err!("Failed to encrypt file"),
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
                                        Err(_) => print_err!("Failed to encrypt file"),
                                    };
                                } else {
                                    match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                        Ok(_) => {
                                            print_success!(
                                                "Encrypted file saved as {:?}!",
                                                output_path
                                            )
                                        }
                                        Err(_) => print_err!("Failed to encrypt file"),
                                    };
                                }
                            }
                            false => print_err!("Failed to get {:?} directory", output_dir),
                        }
                    } else {
                        match file_path.parent() {
                            Some(parent_dir) => {
                                let output_path = encrypted_file_path(&file_path, &parent_dir);
                                match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => {
                                        print_success!("Encrypted file saved as {:?}!", output_path)
                                    }
                                    Err(_) => print_err!("Failed to encrypt file"),
                                };
                            }
                            None => print_err!("Failed to get target file parent directory"),
                        }
                    }
                } else {
                    print_err!("{:?} does not exist!", filepath);
                    print_success!("Check target file and try again");
                    return;
                }
            } else if let true = sub_matches.get_flag("ukey") {
                if let Err(_) = update_encryption_key() {
                    // initialize encryption key if 1st time using command
                    print_err!("Error updating encryption key!");
                    return;
                }
            }
        }
        Some(("decrypt", sub_matches)) => {
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
                                Err(_) => print_err!("Failed to decrypt file"),
                            };
                        } else {
                            match decrypt_file(&file_path.to_path_buf(), &file_path.to_path_buf()) {
                                Ok(_) => {
                                    print_success!(
                                        "{:?} content replaced with decrypted one!",
                                        &file_path
                                    )
                                }
                                Err(_) => print_err!("Failed to decrypt file"),
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
                                        Err(e) => print_err!("Failed to decrypt file: {}", e),
                                    };
                                } else {
                                    match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                        Ok(_) => {
                                            print_success!(
                                                "Decrypted file saved as {:?}!",
                                                output_path
                                            )
                                        }
                                        Err(e) => print_err!("Failed to decrypt file: {}", e),
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
                                        Err(e) => print_err!("Failed to decrypt file: {}", e),
                                    };
                                } else {
                                    match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                        Ok(_) => {
                                            print_success!(
                                                "Decrypted file saved as {:?}!",
                                                output_path
                                            )
                                        }
                                        Err(e) => print_err!("Failed to decrypt file: {}", e),
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
                                        Err(e) => print_err!("Failed to decrypt file: {}", e),
                                    };
                                } else {
                                    match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                        Ok(_) => {
                                            print_success!(
                                                "Decrypted file saved as {:?}!",
                                                output_path
                                            )
                                        }
                                        Err(e) => print_err!("Failed to decrypt file: {}", e),
                                    };
                                }
                            }
                            None => print_err!("Failed to get target file parent directory"),
                        }
                    }
                } else {
                    print_err!("{:?} does not exist!", filepath);
                    print_success!("Check target file and try again");
                    return;
                }
            }
        }
        Some(("password", sub_matches)) => {
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
                                    .with_confirmation(
                                        "Confirm your password",
                                        "The passwords don't match",
                                    )
                                    .interact()
                                    .unwrap()
                            };

                        match set_password(&password) {
                            Ok(_) => {
                                print_success!(
                                    "\"{}\" was successfully set as your new password",
                                    password
                                )
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
                            let actual_password: String =
                                Password::with_theme(&ColorfulTheme::default())
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
                            let actual_password: String =
                                Password::with_theme(&ColorfulTheme::default())
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
                            let chosen: usize =
                                Select::with_theme(&theme::ColorfulTheme::default())
                                    .with_prompt("Choose an option")
                                    .items(&items)
                                    .default(0)
                                    .interact()
                                    .unwrap();

                            if chosen == 0 {
                                match retrieve_questions() {
                                    Ok(questions) => {
                                        println!("Questions List:");
                                        for question in &questions {
                                            println!("• {}", question);
                                        }
                                    }
                                    Err(_) => print_err!("No question set"),
                                }
                            } else if chosen == 1 {
                                let question: String = Input::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Your Question")
                                    .validate_with(|input: &String| -> Result<(), &str> {
                                        let questions: Vec<String> = match retrieve_questions() {
                                            Ok(questions) => questions,
                                            Err(_) => vec![],
                                        };
                                        if questions
                                            .iter()
                                            .any(|line| line == &format!("{}", input))
                                        {
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
                                        if input.len() > 2 {
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
                                let mut choices = match retrieve_questions() {
                                    Ok(questions) => questions,
                                    Err(_) => {
                                        print_err!("No question set");
                                        continue;
                                    }
                                };
                                choices.push("Cancel".to_string());

                                let chosen: usize =
                                    Select::with_theme(&theme::ColorfulTheme::default())
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
                        Ok(mut questions) => {
                            if questions.len() < 3 {
                                print_err!("Not enough questions were set");
                                print_solution!("3 questions are needed to reset password");
                                return;
                            }
                            questions.push("Cancel".to_string());
                            let mut answered_questions: Vec<[String; 2]> = vec![];
                            const QUESTION_KEYWORD: &str = "MUCLI_QUESTION";

                            loop {
                                let chosen: usize =
                                    Select::with_theme(&theme::ColorfulTheme::default())
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

                                questions = match filter_map_lines(|line| -> Option<String> {
                                    if answered_questions.iter().any(|[q, a]| {
                                        line == &format!("{}={}={}", QUESTION_KEYWORD, q, a)
                                    }) {
                                        None
                                    } else if line
                                        == &format!(
                                            "{}={}={}",
                                            QUESTION_KEYWORD, questions[chosen], answer
                                        )
                                    {
                                        answered_questions.push([
                                            questions[chosen].to_owned(),
                                            answer.to_owned(),
                                        ]);
                                        if answered_questions.len() < 3 {
                                            print_success!("That's a right answer");
                                        }
                                        None
                                    } else if line.starts_with(&format!("{}=", QUESTION_KEYWORD)) {
                                        Some(line.split('=').nth(1).unwrap().to_string())
                                    } else {
                                        None
                                    }
                                }) {
                                    Ok(v) => v,
                                    Err(_) => {
                                        print_err!("An error occured!");
                                        return;
                                    }
                                };
                                if answered_questions.len() == 3 {
                                    print_success!(
                                        "That's 3 right answers! Change your password now!"
                                    );
                                    break;
                                }
                                questions.push("Cancel".to_string());
                            }
                            let new_password: String =
                                Password::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Enter your new password")
                                    .with_confirmation(
                                        "Confirm your new password",
                                        "The passwords don't match",
                                    )
                                    .interact()
                                    .unwrap();

                            match set_password(&new_password) {
                                Ok(_) => print_success!(
                                    "New password \"{}\" set successfully!",
                                    new_password
                                ),
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
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
