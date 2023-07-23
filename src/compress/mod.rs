use std::{
    io::Error,
    path::{Path, PathBuf},
};

use clap::ArgMatches;
use zip::result::ZipError;
use zip_extensions::write::zip_create_from_directory;

use custom_error::custom_error;

custom_error! { CompressionError
    Io{source: Error} = "{source}",
    Zip{source: ZipError} = "{source}",
    Default = "Failed to compress file"
}

pub fn compress_command(sub_matches: &ArgMatches) {
    create_zip(&Path::new("test").to_path_buf(), &Path::new("test2.zip").to_path_buf()).unwrap();
}

fn create_zip(source_path: &PathBuf, output_path: &PathBuf) -> Result<(), CompressionError> {
    zip_create_from_directory(&output_path, &source_path).unwrap();

    Ok(())
}
