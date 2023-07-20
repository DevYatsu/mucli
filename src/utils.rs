use std::{
    fs::{File, OpenOptions},
    io::{Error, Read, Seek, SeekFrom, Write},
    num::ParseIntError,
};

use dialoguer::console::Term;
use indicatif::{ProgressBar, ProgressStyle};
const CONFIG_FILE: &str = "config.txt";

extern crate custom_error;
use custom_error::custom_error;

custom_error! {pub GenericError
    Io{source: Error} = "{source}",
    Format{source: ParseIntError} = "{source}",
    KeyNotFound{key: String, filename: String} = "Key \"{key}\" not found in {filename}.",
    Unknown = "unknown error"
}

pub fn arrow_progress(steps: u64) -> ProgressBar {
    let pb = ProgressBar::new(steps);
    pb.set_style(
        ProgressStyle::with_template(
            // note that bar size is fixed unlike cargo which is dynamic
            // and also the truncation in cargo uses trailers (`...`)
            if Term::stdout().size().1 > 20 {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}"
            } else {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
            },
        )
        .unwrap()
        .progress_chars("=> "),
    );

    pb
}

pub fn set_key(new_line: String) -> Result<(), GenericError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(CONFIG_FILE)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file

    file.set_len(0)?;

    for line in buffer.lines() {
        writeln!(file, "{}", line)?;
    }

    writeln!(file, "{}", new_line)?;

    Ok(())
}

pub fn replace_key(keyword: &str, new_line: String) -> Result<(), GenericError> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(CONFIG_FILE)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Create a new buffer with modified lines
    let modified_buffer = buffer
        .lines()
        .filter(|line| !line.starts_with(&format!("{}=", keyword)))
        .chain(std::iter::once(new_line.as_str()))
        .collect::<Vec<&str>>()
        .join("\n");

    // Reset the file pointer to the beginning
    file.seek(SeekFrom::Start(0))?;

    // Write the modified contents to the file
    file.write_all(modified_buffer.as_bytes())?;

    // Truncate any remaining content after the new data
    file.set_len(modified_buffer.len() as u64)?;

    Ok(())
}

pub fn get_keys(keyword: &str) -> Result<Vec<String>, GenericError> {
    let mut file: File = File::open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}{}", keyword, "=")))
        .map(|l| l.split('=').nth(1).unwrap().trim().to_string())
        .collect::<Vec<String>>())
}

pub fn get_key(keyword: &str) -> Result<String, GenericError> {
    let mut file: File = File::open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    for line in buffer.lines() {
        if line.starts_with(&format!("{}{}", keyword, "=")) {
            return Ok(line.split('=').nth(1).unwrap().trim().to_string());
        }
    }
    Err(GenericError::KeyNotFound {
        key: keyword.to_owned(),
        filename: CONFIG_FILE.to_owned(),
    })
}

pub fn filter_map_lines<F, T>(keyword: &str, f: F) -> Result<Vec<T>, GenericError>
where
    F: FnMut(&str) -> T,
{
    let mut file: File = File::open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}=", keyword)))
        .map(f)
        .collect())
}

pub fn key_exists(keyword: &str) -> Result<bool, GenericError> {
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(CONFIG_FILE)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    if buffer
        .lines()
        .any(|line| line.starts_with(&format!("{}=", keyword)))
    {
        // Encryption key already exists, no need to write it again
        return Ok(true);
    }
    Ok(false)
}

pub fn string_as_key<T: std::str::FromStr>(string: &str) -> Result<Vec<T>, T::Err>
where
    T::Err: std::fmt::Debug,
{
    Ok(string
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|val| val.trim().parse::<T>().unwrap())
        .collect::<Vec<T>>())
}
