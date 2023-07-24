mod annex;

use crate::compression::annex::create_zip;
use std::{
    env::current_dir,
    fs,
    io::Error,
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use zip::result::ZipError;

use custom_error::custom_error;

use crate::{print_err, print_success};

use self::annex::extract_zip;

custom_error! {pub CompressionError
    Io{source: Error} = "{source}",
    Zip{source: ZipError} = "{source}",
    Default = "Failed to compress file",
    Custom{src: String} = "{src}"
}

pub fn compress_command(sub_matches: &ArgMatches) {
    if let Some(source_path) = sub_matches.get_one::<PathBuf>("PATH") {
        let source_path = PathBuf::from(source_path);

        let source_name = match fs::canonicalize(&source_path) {
            Ok(p) => p
                .file_name()
                .to_owned()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            Err(e) => {
                print_err!("(compression error): {}", e);
                return;
            }
        };
        let output_file_name = format!("{}.zip", source_name);

        let compression_level = sub_matches
            .get_one::<i64>("level")
            .copied()
            .map(|val| val as i32);

        if let true = sub_matches.get_flag("cdir") {
            match current_dir() {
                Ok(current_dir) => {
                    let output_path = current_dir.join(&output_file_name);
                    match create_zip(&source_path, &output_path, compression_level) {
                        Ok(_) => print_success!(
                            "{} successfully compressed as {}",
                            source_path.display(),
                            output_path.display()
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
                        "{} successfully compressed as {}",
                        source_path.display(),
                            output_path.display()
                    ),
                    Err(e) => print_err!("(compress error): {}", e),
                },
                false => print_err!("Failed to get {} directory", output_dir.display()),
            }
        } else {
            match source_path.parent() {
                Some(parent_dir) => {
                    let output_path = &Path::new(parent_dir).join(output_file_name);
                    match create_zip(&source_path, &output_path, compression_level) {
                        Ok(_) => print_success!(
                            "{} successfully compressed as {}",
                            source_path.display(),
                            output_path.display()
                        ),
                        Err(e) => print_err!("(compress error): {}", e),
                    }
                }
                None => print_err!("Failed to get source directory parent directory"),
            }
        }
    }
}

pub fn extract_command(sub_matches: &ArgMatches) {
    if let Some(source_path) = sub_matches.get_one::<PathBuf>("PATH") {
        let source_path = Path::new(source_path).to_path_buf();
        let source_path = match fs::canonicalize(&source_path) {
            Ok(p) => p,
            Err(e) => {
                print_err!("(compression error): {}", e);
                return;
            }
        };
        
        if let true = sub_matches.get_flag("cdir") {
            match current_dir() {
                Ok(current_dir) => match extract_zip(&source_path, &current_dir) {
                    Ok(_) => print_success!(
                        "{} successfully extracted in {}",
                        source_path.display(),
                        current_dir.display()
                    ),
                    Err(e) => print_err!("(extraction error): {}", e),
                },
                Err(error) => {
                    print_err!("Failed to get current directory: {}", error)
                }
            }
        } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
            match output_dir.is_dir() {
                true => match extract_zip(&source_path, &output_dir) {
                    Ok(_) => print_success!(
                        "{} successfully extracted in {}",
                        source_path.display(),
                        output_dir.display()
                    ),
                    Err(e) => print_err!("(extraction error): {}", e),
                },
                false => print_err!("Failed to get {} directory", output_dir.display()),
            }
        } else {
            match source_path.parent() {
                Some(parent_dir) => match extract_zip(&source_path, &parent_dir.to_path_buf()) {
                    Ok(_) => print_success!(
                        "{} successfully extracted in {}",
                        source_path.display(),
                        parent_dir.display()
                    ),
                    Err(e) => print_err!("(extraction error): {}", e),
                },
                None => print_err!("Failed to get source directory parent directory"),
            }
        }
    }
}
