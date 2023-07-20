mod encryption;
mod password;
mod utils;

use clap::{arg, command, ArgAction, ArgGroup, Command};
use dialoguer::{theme::ColorfulTheme, Password};
use encryption::{
    decrypted_file_path, encrypted_file_path, init_encryption_key, update_encryption_key,
    update_file_encryption_key,
};
use password::{get_password, init_password_key, set_password};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::encryption::{decrypt_file, encrypt_file};

macro_rules! print_err {
    ($fmt:literal) => (println!("\x1B[38;5;196merror\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;196merror\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
macro_rules! print_advice {
    ($fmt:literal) => (println!("\x1B[38;5;227msolution\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;227msolution\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
macro_rules! print_success {
    ($fmt:literal) => (println!("\x1B[38;5;46minfo\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;46minfo\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
macro_rules! print_future_update {
    ($fmt:literal) => (println!("\x1B[38;5;57mupcoming\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;57mupcoming\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("password")
                .about("Set a security password to access sensible informations")
                .group(
                    ArgGroup::new("password_action")
                        .required(true)
                        .args(["init", "change", "reset"]),
                )
                .arg(
                    arg!(-'i' --"init" [NEW_PASSWORD] "Set password for first time")
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-'c' --"change"  [INITIAL_PASSWORD] "Change password when set")
                        .action(ArgAction::Set),
                )                .arg(
                    arg!(-'r' --"reset" "Reset password (future release)")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts the specified file and place the output file in specified dir")
                .group(
                    ArgGroup::new("encrypt_actions")
                        .required(false)
                        .args(["ukey", "cdir"]),
                )
                .arg(
                    arg!(-'u' --"ukey" "Update encryption key or update encryption key of a file to the latest version")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-'c' --"cdir" "Place output file in current dir")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!([FILEPATH] "file path of the target file")
                        .required_unless_present("ukey")
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    arg!([OUTPUTDIR] "output directory [defaults: file dir]")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts the specified file and place the output file in specified dir")
                .arg(
                    arg!(-'c' --"cdir" "Place output file in current dir")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!([FILEPATH] "file path of the target file")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    arg!([OUTPUTDIR] "output directory [defaults: file dir]")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
                if let Err(_) = init_encryption_key() {
                    // initialize encryption key if 1st time using command
                    print_err!("Initializing encryption key failed!");
                    return;
                }
                let file_path: &Path = Path::new(filepath);
                if file_path.exists() {
                    if let true = sub_matches.get_flag("cdir") {
                        match current_dir() {
                            Ok(current_dir) => {
                                let output_path = encrypted_file_path(&file_path, &current_dir);
                                match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => {
                                        print_success!("Encrypted file saved as {:?}!", output_path)
                                    }
                                    Err(_) => print_err!("Failed to encrypt file"),
                                };
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
                            Err(e) => print_err!("Key updating failed: {}", e),
                        }
                    } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                        match Path::new(output_dir).is_dir() {
                            true => {
                                let output_path = encrypted_file_path(&file_path, &output_dir);
                                match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => {
                                        print_success!("Encrypted file saved as {:?}!", output_path)
                                    }
                                    Err(_) => print_err!("Failed to encrypt file"),
                                };
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
                    print_advice!("Check target file and try again");
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
                    if let true = sub_matches.get_flag("cdir") {
                        match current_dir() {
                            Ok(current_dir) => {
                                let output_path = decrypted_file_path(&file_path, &current_dir);
                                match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => {
                                        print_success!("Decrypted file saved as {:?}!", output_path)
                                    }
                                    Err(e) => print_err!("{}", e),
                                };
                            }
                            Err(error) => {
                                print_err!("Failed to get current directory: {}", error)
                            }
                        }
                    } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                        match Path::new(output_dir).is_dir() {
                            true => {
                                let output_path = decrypted_file_path(&file_path, &output_dir);
                                match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => {
                                        print_success!("Decrypted file saved as {:?}!", output_path)
                                    }
                                    Err(e) => print_err!("{}", e),
                                };
                            }
                            false => print_err!("Failed to get {:?} directory", output_dir),
                        }
                    } else {
                        match file_path.parent() {
                            Some(parent_dir) => {
                                let output_path = decrypted_file_path(&file_path, &parent_dir);
                                match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => {
                                        print_success!("Decrypted file saved as {:?}!", output_path)
                                    }
                                    Err(e) => print_err!("{}", e),
                                };
                            }
                            None => print_err!("Failed to get target file parent directory"),
                        }
                    }
                } else {
                    print_err!("{:?} does not exist!", filepath);
                    print_advice!("Check target file and try again");
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
                        print_advice!("Use --change|-c flag to modify it");
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
                                        "The passwords do not match",
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
                            Err(e) => print_err!("Failed to set password! -> {}", e),
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
                            } else {
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
                                )
                            }
                            Err(e) => print_err!("{}", e),
                        };
                    }
                    Err(_) => {
                        print_err!("Password has not been set yet");
                        print_advice!("Use --init|-i flag to set it")
                    }
                }
            } else if let true = sub_matches.contains_id("reset") {
                print_err!("Impossible to reset password");
                print_future_update!("Feature coming in the next release!")
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
