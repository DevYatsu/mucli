mod annex;

use std::path::PathBuf;

use clap::ArgMatches;

use crate::{print_err, print_success};

use self::annex::copy;

pub fn copy_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if let Some(target) = sub_matches.get_one::<PathBuf>("TARGET") {
            match copy(filepath, target) {
                Ok(_) => print_success!("{:?} was copied in {:?} successfully", filepath, target),
                Err(e) => print_err!("(copy failed): {}", e),
            }
        }
    }
}
