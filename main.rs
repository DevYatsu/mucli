mod encryption;
mod password;

use clap::{arg, command, Arg, ArgAction, ArgGroup, Command};
use dialoguer::{theme::ColorfulTheme, Password};
use encryption::{
    decrypted_file_path, encrypted_file_path, init_encryption_key, update_encryption_key,
};
use password::{get_password, set_password};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::encryption::{decrypt_file, encrypt_file};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(Arg::new("hey").long("hey").action(ArgAction::SetTrue))
        .subcommand(
            Command::new("password")
                .about("Set a security password to access sensible informations")
                .group(
                    ArgGroup::new("password_action")
                        .required(true)
                        .args(["init", "change"]),
                )
                .arg(
                    arg!(-'i' --"init" [NEW_PASSWORD] "Set password for first time")
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-'c' --"change"  [INITIAL_PASSWORD] "Change password when set")
                        .action(ArgAction::Set),
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
                .arg(arg!(-'u' --"ukey" "Update encryption key. Advised to use from time to time."))
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
            if let Err(_) = init_encryption_key() {
                // initialize encryption key if 1st time using command
                eprintln!("Error initializing encryption key!");
                return;
            }
            if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
                let file_path: &Path = Path::new(filepath);
                if file_path.exists() {
                    if let true = sub_matches.get_flag("cdir") {
                        match current_dir() {
                            Ok(current_dir) => {
                                let output_path = encrypted_file_path(&file_path, &current_dir);
                                match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => println!("Decrypted file saved as {:?}!", output_path),
                                    Err(_) => eprintln!("Failed to decrypt file."),
                                };
                            }
                            Err(error) => {
                                eprintln!("Failed to get current directory: {}.", error)
                            }
                        }
                    } else if let true = sub_matches.get_flag("ukey") {
                        if let Err(_) = update_encryption_key() {
                            // initialize encryption key if 1st time using command
                            eprintln!("Error updating encryption key!");
                            return;
                        }
                    } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                        match Path::new(output_dir).is_dir() {
                            true => {
                                let output_path = encrypted_file_path(&file_path, &output_dir);
                                match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => println!("Decrypted file saved as {:?}!", output_path),
                                    Err(_) => eprintln!("Failed to decrypt file."),
                                };
                            }
                            false => eprintln!("Failed to get {:?} directory.", output_dir),
                        }
                    } else {
                        match file_path.parent() {
                            Some(parent_dir) => {
                                let output_path = encrypted_file_path(&file_path, &parent_dir);
                                match encrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => println!("Decrypted file saved as {:?}!", output_path),
                                    Err(_) => eprintln!("Failed to decrypt file."),
                                };
                            }
                            None => eprintln!("Failed to get target file parent directory."),
                        }
                    }
                } else {
                    println!(
                        "{:?} does not exist!\nCheck target file and try again.",
                        filepath
                    );
                    return;
                }
            }
        }
        Some(("decrypt", sub_matches)) => {
            if let Err(_) = init_encryption_key() {
                // initialize encryption key if 1st time using command
                eprintln!("Error initializing encryption key!");
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
                                    Ok(_) => println!("Decrypted file saved as {:?}!", output_path),
                                    Err(_) => eprintln!("Failed to decrypt file."),
                                };
                            }
                            Err(error) => {
                                eprintln!("Failed to get current directory: {}.", error)
                            }
                        }
                    } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                        match Path::new(output_dir).is_dir() {
                            true => {
                                let output_path = decrypted_file_path(&file_path, &output_dir);
                                match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => println!("Decrypted file saved as {:?}!", output_path),
                                    Err(_) => eprintln!("Failed to decrypt file."),
                                };
                            }
                            false => eprintln!("Failed to get {:?} directory.", output_dir),
                        }
                    } else {
                        match file_path.parent() {
                            Some(parent_dir) => {
                                let output_path = decrypted_file_path(&file_path, &parent_dir);
                                match decrypt_file(&file_path.to_path_buf(), &output_path) {
                                    Ok(_) => println!("Decrypted file saved as {:?}!", output_path),
                                    Err(_) => eprintln!("Failed to decrypt file."),
                                };
                            }
                            None => eprintln!("Failed to get target file parent directory."),
                        }
                    }
                } else {
                    println!(
                        "{:?} does not exist!\nCheck target file and try again.",
                        filepath
                    );
                    return;
                }
            }
        }
        Some(("password", sub_matches)) => {
            if let true = sub_matches.contains_id("init") {
                match get_password() {
                    Ok(_) => println!("Password is already set. Use --change flag to modify it."),
                    Err(_) => {
                        let password: String =
                            if let Some(new_password) = sub_matches.get_one::<String>("init") {
                                Password::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Confirm your password")
                                    .validate_with(|input: &String| -> Result<(), &str> {
                                        if input != new_password {
                                            return Err("Passwords don't match.");
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
                                        "The passwords do not match.",
                                    )
                                    .interact()
                                    .unwrap()
                            };

                        match set_password(&password) {
                            Ok(_) => println!("{password} successfully set as your password."),
                            Err(e) => eprintln!("Failed to set password! Please try again.{e}"),
                        };
                    }
                }
            } else if let true = sub_matches.contains_id("change") {
                match get_password() {
                    Ok(password) => {
                        if let Some(password_input) = sub_matches.get_one::<String>("change") {
                            if *password_input != password {
                                println!("Invalid password put as argument!");
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
                                println!("Wrong password!");
                                return;
                            }
                        }

                        let new_password: String = Password::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter your new password")
                            .with_confirmation(
                                "Confirm your new password",
                                "Passwords don't match.",
                            )
                            .interact()
                            .unwrap();

                        match set_password(&new_password) {
                            Ok(_) => {
                                println!(
                                    "\"{new_password}\" was successfully set as your new password."
                                )
                            }
                            Err(_) => eprintln!("An error occured! Try again."),
                        };
                    }
                    Err(_) => {
                        eprintln!("Password has not been set yet. Use --init flag to set it.")
                    }
                }
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}