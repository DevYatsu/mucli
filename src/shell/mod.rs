mod annex;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::{print_err, print_success};

use self::annex::execute_shell_script;

pub fn shell_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if !filepath.is_file() {
            print_err!("Source path must be a file!");
            return;
        }

        match execute_shell_script(filepath) {
            Ok(_) => print_success!("Shell script execution successful"),
            Err(e) => print_err!("(execution error): {}", e),
        };
    }
}
