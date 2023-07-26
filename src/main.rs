mod antivirus;
mod compression;
mod copy;
mod encryption;
mod r#move;
mod password;
mod rename;
mod shell;
mod timer;
mod update;
mod utils;

use crate::antivirus::antivirus_command;
use crate::compression::compress_command;
use crate::copy::copy_command;
use crate::r#move::move_command;
use crate::rename::rename_command;
use crate::shell::shell_command;
use crate::timer::timer_command;
use crate::update::update_command;
use crate::utils::config_interact::Config;
use clap::{arg, command, ArgAction, ArgGroup, Command};

use compression::extract_command;
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
                        .args(["ukey", "cdir", "sfile", "purge", "OUTPUTDIR"]),                    
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
                        .args(["cdir", "sfile", "OUTPUTDIR"]),
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
            Command::new("cp")
                .about("Copy a file content into another existing or non-existing file or into a directory")
                .arg(arg!([FILEPATH] "file path of the target file").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([TARGET] "new file directory").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("mv")
                .about("Move a file into a directory")
                .arg(arg!([FILEPATH] "file path of the target file").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([DIR] "target directory [defaults: current dir]").value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("zip")
                .about("Compress the specified file/directory and place the output file in specified dir")
                .group(
                    ArgGroup::new("compress_actions")
                        .required(false)
                        .args(["cdir", "OUTPUTDIR"])
                )
                .arg(arg!(-'c' --"cdir" "Place output zip in current dir").action(ArgAction::SetTrue))
                .arg(arg!(-'l' --"level" <LEVEL> "Compress using a specified compression level between 0 and 9").action(ArgAction::Set).value_parser(0..=9))
                .arg(arg!([PATH] "path of the source to compress").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([OUTPUTDIR] "output directory [defaults: file dir]").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("unzip")
                .about("Extract the specified zip and place the output extract in specified dir")
                .group(
                    ArgGroup::new("compress_actions")
                        .required(false)
                        .args(["cdir", "OUTPUTDIR"])
                )
                .arg(arg!(-'c' --"cdir" "Place output extract in current dir").action(ArgAction::SetTrue))
                .arg(arg!([PATH] "path of the zip to extract").required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!([OUTPUTDIR] "output directory [defaults: file dir]").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("shell")
                .about("Execute a shell script")
                .arg(arg!([FILEPATH] "path to the script").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("antivirus")
                .about("Check for malwares in a given file, using virustotal API")
                .long_about(
                    "Check for malwares in a given file, using virustotal API.
This command returns:
        - malicious threat detected: that is the number of engine reports saying the file is malicious
        - suspicious threat detected: that is the number of engine reports saying the file is suspicious
                    "
                )
                .arg(arg!([FILEPATH] "path to the file").required(true).value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("timer")
                .about("Just a simple timer")
        )
        .subcommand(
            Command::new("update")
                .about("Check if a new update of mucli is available (coming soon)")
        ).get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => encrypt_command(sub_matches),
        Some(("decrypt", sub_matches)) => decrypt_command(sub_matches),
        Some(("password", sub_matches)) => password_command(sub_matches),
        Some(("update", _)) => update_command().await,
        Some(("rename", sub_matches)) => rename_command(sub_matches),
        Some(("cp", sub_matches)) => copy_command(sub_matches),
        Some(("mv", sub_matches)) => move_command(sub_matches),
        Some(("zip", sub_matches)) => compress_command(sub_matches),
        Some(("unzip", sub_matches)) => extract_command(sub_matches),
        Some(("timer", _)) => timer_command(),
        Some(("shell", sub_matches)) => shell_command(sub_matches),
        Some(("antivirus", sub_matches)) => antivirus_command(sub_matches).await,
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
