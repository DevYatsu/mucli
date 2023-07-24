use std::{
    fs::{self, File},
    io::{self, Read},
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
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .compression_level(compression_level)
        .unix_permissions(0o755);

    if source_path.is_dir() {
        let mut path_queue = vec![];
        path_queue.push(source_path.to_owned());

        while let Some(dir) = path_queue.pop() {
            let entries = fs::read_dir(&dir)?;

            for (i, entry) in entries.enumerate() {
                let entry = entry?;
                let entry_path = entry.path();
                let entry_name = entry_path.to_string_lossy().to_string();

                if entry_path.is_file() {
                    let (_, content) = file_as_bytes!(&entry_path);

                    {
                        let path = entry_path;
                        println!("File {i} comment: {:?}", path);
                    }
                    
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

    Ok(())
}

pub fn extract_zip(source_path: &PathBuf, output_dir: &PathBuf) -> Result<(), CompressionError> {
    let source_file = File::open(source_path)?;

    let mut archive = zip::ZipArchive::new(source_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let outpath = output_dir.join(outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}
