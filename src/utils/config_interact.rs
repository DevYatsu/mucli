const CONFIG_FILE: &str = "config.txt";

use std::io::{Seek, SeekFrom, Write};

use crate::{file, parse_config_line};

use super::GenericError;

pub fn set_key(new_line: String) -> Result<(), GenericError> {
    let (mut file, buffer) = file!(CONFIG_FILE);

    file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file

    file.set_len(0)?;

    for line in buffer.lines() {
        writeln!(file, "{}", line)?;
    }

    writeln!(file, "{}", new_line)?;

    Ok(())
}

pub fn replace_key(keyword: &str, new_line: String) -> Result<(), GenericError> {
    let (mut file, buffer) = file!(CONFIG_FILE);

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
    let (_, buffer) = file!(CONFIG_FILE);

    Ok(buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}{}", keyword, "=")))
        .map(|l| parse_config_line!(l).unwrap().into_iter().nth(1).unwrap())
        .collect::<Vec<String>>())
}

pub fn get_key(keyword: &str) -> Result<String, GenericError> {
    let (_, buffer) = file!(CONFIG_FILE);

    for line in buffer.lines() {
        if line.starts_with(&format!("{}{}", keyword, "=")) {
            return Ok(parse_config_line!(line).unwrap().into_iter().nth(1).unwrap());
        }
    }
    Err(GenericError::KeyNotFound {
        key: keyword.to_owned(),
        filename: CONFIG_FILE.to_owned(),
    })
}

//remove filter and map to only let filter_map
pub fn filter_and_map_lines<F, T>(keyword: &str, f: F) -> Result<Vec<T>, GenericError>
where
    F: FnMut(&str) -> T,
{
    let (_, buffer) = file!(CONFIG_FILE);

    Ok(buffer
        .lines()
        .filter(|line| line.starts_with(&format!("{}=", keyword)))
        .map(f)
        .collect())
}
pub fn filter_map_lines<F, T>(f: F) -> Result<Vec<T>, GenericError>
where
    F: FnMut(&str) -> Option<T>,
{
    let (_, buffer) = file!(CONFIG_FILE);

    Ok(buffer.lines().filter_map(f).collect())
}

pub fn key_exists(keyword: &str) -> Result<bool, GenericError> {
    let (_, buffer) = file!(CONFIG_FILE);

    if buffer
        .lines()
        .any(|line| line.starts_with(&format!("{}=", keyword)))
    {
        // Encryption key already exists, no need to write it again
        return Ok(true);
    }
    Ok(false)
}

pub fn string_as_vec<T: std::str::FromStr>(string: &str) -> Result<Vec<T>, T::Err>
where
    T::Err: std::fmt::Debug,
{
    Ok(string
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|val| val.trim().parse::<T>().unwrap())
        .collect::<Vec<T>>())
}
pub fn vec_as_string<T: ToString>(vec: Vec<T>) -> String {
    format!("[{}]", vec.into_iter().map(|val| val.to_string()).collect::<Vec<_>>().join(","))
}