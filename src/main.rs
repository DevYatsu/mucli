mod compress;
mod copy;
mod encryption;
mod r#move;
mod password;
mod rename;
mod update;
mod utils;

use crate::compress::compress_command;
use crate::copy::copy;
use crate::r#move::mv;
use crate::rename::rename;
use crate::utils::config_interact::Config;
use clap::{arg, command, ArgAction, ArgGroup, Command};

use encryption::{decrypt_command, encrypt_command};
use password::password_command;
use std::path::PathBuf;

const VERSION: &str = "v1.0.0";

#[tokio::main]
async fn main() {
    let matches = command!()
        .author("yatsu")
        .name("mucli")
        .version(VERSION)
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
                .about("Encrypt the specified file and place the output file in specified dir")
                .groups([
                    ArgGroup::new("encrypt_actions")
                        .required(false)
                        .args(["ukey", "cdir", "sfile", "purge"]),                    
                    ArgGroup::new("additional_actions")
                        .required(false)
                        .args(["times", "ukey", "purge"])
                ])
                .arg(arg!(-'u' --"ukey" "Update encryption key or update encryption key of a file to the latest version").action(ArgAction::SetTrue))
                .arg(arg!(-'c' --"cdir" "Place output file in current dir").action(ArgAction::SetTrue))
                .arg(arg!(-'s' --"sfile" "Select target file as output file").action(ArgAction::SetTrue))
                .arg(arg!(-'p' --"purge" "Get rid of all the encryption keys to start anew").action(ArgAction::SetTrue))
                .arg(arg!(-'t' --"times" <TIMES> "Encrypt x times the file").action(ArgAction::Set).value_parser(clap::value_parser!(u8)))
                .arg(arg!([FILEPATH] "file path of the target file").required_unless_present_any(["ukey", "purge"]).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([OUTPUTDIR] "output directory [defaults: file dir]").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt the specified file and place the output file in specified dir")
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
        .subcommand(
            Command::new("rename")
                .about("Rename a file as specified")
                .arg(arg!([FILEPATH] "file path of the target file").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([NAME] "new file name").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("copy")
                .about("Copy a file content into another existing or non-existing file or into a directory")
                .arg(arg!([FILEPATH] "file path of the target file").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([TARGET] "new file directory").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("move")
                .about("Move a file into a directory")
                .arg(arg!([FILEPATH] "file path of the target file").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([DIR] "target directory").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("compress")
                .about("Compress the specified file and place the output file in specified dir")
                .arg(arg!(-'c' --"cdir" "Place output file in current dir").action(ArgAction::SetTrue))
                .arg(arg!([DIRPATH] "path of the directory to compress").value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([OUTPUTDIR] "output directory [defaults: file dir]").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("update")
                .about("Check if a new update of mucli is available (coming soon)")
        ).get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => encrypt_command(sub_matches),
        Some(("decrypt", sub_matches)) => decrypt_command(sub_matches),
        Some(("password", sub_matches)) => password_command(sub_matches),
        Some(("update", _)) => {
            print_future_update!("Command coming soon!");
            return;
            // match get_latest_release_version().await {
            //     Ok(v) => {
            //         print_success!("Latest release version is \"{}\"", v);
            //         if can_update(VERSION, &v) {
            //             print_success!("This version is superior to current version \"{}\"", v);
            //             let confirmation = Confirm::with_theme(&ColorfulTheme::default())
            //                 .with_prompt("Would you like to upgrade to the latest version?")
            //                 .interact()
            //                 .unwrap();

            //             if !confirmation {
            //                 return;
            //             }
            //             //update version
            //         }
            //     }
            //     Err(e) => print_err!("{}", e),
            // };
        }
        Some(("rename", sub_matches)) => {
            if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
                if let Some(new_name) = sub_matches.get_one::<PathBuf>("NAME") {
                    match rename(filepath, new_name) {
                        Ok(_) => {
                            print_success!("{:?} renamed {:?} successfully", filepath, new_name)
                        }
                        Err(e) => print_err!("(renaming failed): {}", e),
                    }
                }
            }
        }
        Some(("copy", sub_matches)) => {
            if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
                if let Some(target) = sub_matches.get_one::<PathBuf>("TARGET") {
                    match copy(filepath, target) {
                        Ok(_) => {
                            print_success!("{:?} was copied in {:?} successfully", filepath, target)
                        }
                        Err(e) => print_err!("(copy failed): {}", e),
                    }
                }
            }
        }
        Some(("move", sub_matches)) => {
            if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
                if let Some(dir) = sub_matches.get_one::<PathBuf>("DIR") {
                    match mv(filepath, dir) {
                        Ok(_) => {
                            print_success!("{:?} was moved in {:?} successfully", filepath, dir)
                        }
                        Err(e) => print_err!("(operation failure): {}", e),
                    }
                }
            }
        }
        Some(("compress", sub_matches)) => compress_command(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
