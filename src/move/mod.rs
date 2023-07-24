mod annex;

use std::path::PathBuf;

use clap::ArgMatches;

use crate::{print_err, print_success};

use self::annex::mv;

pub fn move_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if let Some(dir) = sub_matches.get_one::<PathBuf>("DIR") {
            match mv(filepath, dir) {
                Ok(_) => {
                    print_success!("{:?} was moved in {:?} successfully", filepath, dir)
                }
                Err(e) => print_err!("(operation failure): {}", e),
            }
        } else {
            match mv(filepath, &PathBuf::from(".")) {
                Ok(_) => {
                    print_success!("{:?} was moved in current dir successfully", filepath)
                }
                Err(e) => print_err!("(operation failure): {}", e),
            }
        }
    }
}
