use crate::{print_solution};
use std::{
    env::current_dir,
    fs::{self, File},
    io::{Error, Write, self},
    path::{Path, PathBuf},
};
use clap::ArgMatches;
use zip::result::ZipError;
use crate::file_as_bytes;
use zip::{write::FileOptions, ZipWriter};
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
        let source_path = PathBuf::from(source_path);

        if source_path == PathBuf::from(".") {
            print_err!("Cannot compress directory when inside of it");
            print_solution!("Use \"cd ..\" and try again");
            return;
        }

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
        if !source_path.is_file() {
            print_err!("Source path must be a file!");
            return;
        }

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
                println!("{}", entry_path.display());
                println!("{}", source_path.display());

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
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        println!("{}", outpath.display());
        println!("{}", source_path.display());
        let outpath = output_dir.join(outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
