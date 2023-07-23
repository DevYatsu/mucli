mod annex;

use crate::compression::annex::create_zip;
use std::{
    env::current_dir,
    io::Error,
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use zip::result::ZipError;

use custom_error::custom_error;

use crate::{print_err, print_success};

custom_error! {pub CompressionError
    Io{source: Error} = "{source}",
    Zip{source: ZipError} = "{source}",
    Default = "Failed to compress file",
    Custom{src: String} = "{src}"
}

pub fn compress_command(sub_matches: &ArgMatches) {
    if let Some(source_path) = sub_matches.get_one::<PathBuf>("PATH") {
        let source_path = Path::new(source_path).to_path_buf();

        let source_name = source_path.file_name().unwrap();
        let output_file_name = format!("{}.zip", source_name.to_string_lossy());

        let compression_level = sub_matches.get_one::<i32>("level").copied();

        if let true = sub_matches.get_flag("cdir") {
            match current_dir() {
                Ok(current_dir) => {
                    let output_path = current_dir.join(&output_file_name);
                    match create_zip(&source_path, &output_path, compression_level) {
                        Ok(_) => print_success!(
                            "{:?} successfully compressed as {:?}",
                            source_path,
                            output_path
                        ),
                        Err(e) => print_err!("(compress error): {}", e),
                    }
                }
                Err(error) => {
                    print_err!("Failed to get current directory: {}", error)
                }
            }
        } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
            let output_path = output_dir.join(output_file_name);
            match output_dir.is_dir() {
                true => match create_zip(&source_path, &output_path, compression_level) {
                    Ok(_) => print_success!(
                        "{:?} successfully compressed as {:?}",
                        source_path,
                        output_path
                    ),
                    Err(e) => print_err!("(compress error): {}", e),
                },
                false => print_err!("Failed to get {:?} directory", output_dir),
            }
        } else {
            match source_path.parent() {
                Some(parent_dir) => {
                    let output_path = &Path::new(parent_dir).join(output_file_name);
                    match create_zip(&source_path, &output_path, compression_level) {
                        Ok(_) => print_success!(
                            "{:?} successfully compressed as {:?}",
                            source_path,
                            output_path
                        ),
                        Err(e) => print_err!("(compress error): {}", e),
                    }
                }
                None => print_err!("Failed to get source directory parent directory"),
            }
        }
    }
}
