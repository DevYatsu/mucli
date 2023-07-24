mod annex;

use std::path::PathBuf;

use clap::ArgMatches;

use crate::{print_err, print_success};

use self::annex::rename;

pub fn rename_command(sub_matches: &ArgMatches) {
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
