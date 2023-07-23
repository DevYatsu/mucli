use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::file_as_bytes;

use super::CompressionError;
use std::io::Write;
use zip::{write::FileOptions, ZipWriter};

pub fn create_zip(
    source_path: &PathBuf,
    output_path: &PathBuf,
    compression_level: Option<i32>,
) -> Result<(), CompressionError> {
    let mut buf = [0; 65536];
    let cursor = std::io::Cursor::new(&mut buf[..]);
    let mut zip = ZipWriter::new(cursor);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .compression_level(compression_level);

    if source_path.is_dir() {
        let mut path_queue = vec![];
        path_queue.push(source_path.to_owned());

        while let Some(dir) = path_queue.pop() {
            let entries = fs::read_dir(&dir)?;

            for entry in entries {
                let entry = entry?;
                let entry_path = entry.path();
                let entry_name = entry_path.to_string_lossy().to_string();

                if entry_path.is_file() {
                    let (_, content) = file_as_bytes!(&entry_path);
                    zip.start_file(entry_name, options)?;
                    zip.write(&content)?;
                } else if entry_path.is_dir() {
                    zip.add_directory(entry_name, options)?;
                    path_queue.push(entry_path)
                }
            }
        }
    } else if source_path.is_file() {
        let (_, content) = file_as_bytes!(&source_path);

        if let Some(name) = source_path.file_name() {
            zip.start_file(name.to_string_lossy(), options)?;
            zip.write(&content)?;
        } else {
            return Err(CompressionError::Custom {
                src: "Invalid file path".to_string(),
            });
        }
    }

    zip.finish()?;
    drop(zip);

    let mut file = File::create(output_path)?;
    file.write_all(&buf)?;

    Ok(())
}
