mod annex;
mod responses;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::{print_err, print_info};

use self::annex::post_file;

pub async fn antivirus_command(sub_matches: &ArgMatches) {
    if let Some(filepath) = sub_matches.get_one::<PathBuf>("FILEPATH") {
        if !filepath.is_file() {
            print_err!("Source path must be a file!");
            return;
        }

        match post_file(filepath).await {
            Ok(b) => match b {
                true => {
                    print_info!("File can be considered safe!");
                }
                false => {
                    print_info!("File can be considered dangerous!");
                    print_info!("Further investigations may be conducted...")
                }
            },
            Err(e) => print_err!("(api response): {}", e),
        };
    }
}
