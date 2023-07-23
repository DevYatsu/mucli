use clap::ArgMatches;
use std::io::{Error, Write};
use zip::{result::ZipError, write::FileOptions};

use custom_error::custom_error;

custom_error! { CompressionError
    Io{source: Error} = "{source}",
    Zip{source: ZipError} = "{source}",
    Default = "Failed to compress file"
}

pub fn compress_command(sub_matches: &ArgMatches) {
    write_file("test.txt").unwrap()
}

fn write_file(name: &str) -> Result<(), CompressionError> {
    // We use a buffer here, though you'd normally use a `File`
    let mut buf = [0; 65536];
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf[..]));

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file(name, options)?;
    zip.write(b"Hello, World!")?;

    // Apply the changes you've made.
    // Dropping the `ZipWriter` will have the same effect, but may silently fail
    zip.finish()?;
    Ok(())
}
