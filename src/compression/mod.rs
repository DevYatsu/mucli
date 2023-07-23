use std::{
    env::current_dir,
    io::Error,
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use zip::result::ZipError;
use zip_extensions::write::zip_create_from_directory;

use custom_error::custom_error;

use crate::{print_err, print_solution, print_success};

custom_error! { CompressionError
    Io{source: Error} = "{source}",
    Zip{source: ZipError} = "{source}",
    Default = "Failed to compress file",
    Custom{src: String} = "{src}"
}

pub fn compress_command(sub_matches: &ArgMatches) {
    if let Some(source_path) = sub_matches.get_one::<PathBuf>("DIRPATH") {
        let source_dir: &Path = Path::new(source_path);
        if source_dir.is_dir() {
            let source_dir = source_dir.to_path_buf();
            let dir_name = source_dir.file_name().unwrap();
            let output_file_name = format!("{}.zip", dir_name.to_string_lossy());

            if let true = sub_matches.get_flag("cdir") {
                match current_dir() {
                    Ok(current_dir) => {
                        match create_zip(&source_dir, &current_dir.join(output_file_name)) {
                            Ok(_) => print_success!(
                                "{:?} successfully compressed as {:?}",
                                source_dir,
                                current_dir
                            ),
                            Err(e) => print_err!("(compress error): {}", e),
                        }
                    }
                    Err(error) => {
                        print_err!("Failed to get current directory: {}", error)
                    }
                }
            } else if let Some(output_dir) = sub_matches.get_one::<PathBuf>("OUTPUTDIR") {
                match output_dir.is_dir() {
                    true => match create_zip(&source_dir, &output_dir.join(output_file_name)) {
                        Ok(_) => print_success!(
                            "{:?} successfully compressed as {:?}",
                            source_dir,
                            output_dir
                        ),
                        Err(e) => print_err!("(compress error): {}", e),
                    },
                    false => print_err!("Failed to get {:?} directory", output_dir),
                }
            } else {
                match source_dir.parent() {
                    Some(parent_dir) => {
                        match create_zip(&source_dir, &Path::new(parent_dir).join(output_file_name)) {
                            Ok(_) => print_success!(
                                "{:?} successfully compressed as {:?}",
                                source_dir,
                                parent_dir
                            ),
                            Err(e) => print_err!("(compress error): {}", e),
                        }
                    }
                    None => print_err!("Failed to get source directory parent directory"),
                }
            }
        } else {
            print_err!("{:?} is not a valid directory!", source_dir);
            print_solution!("Check source directory and try again");
            return;
        }
    }
}

fn create_zip(source_path: &PathBuf, output_path: &PathBuf) -> Result<(), CompressionError> {
    match zip_create_from_directory(&output_path, &source_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(CompressionError::Custom { src: e.to_string() }),
    }
}
