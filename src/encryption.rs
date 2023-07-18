use std::fs::File;
use std::io::{Read, Write};

fn encrypt_file(input_path: &str, output_path: &str) -> Result<(), std::io::Error> {
    let key: &[u8; 16] = b"0123456789012345";
    let iv: &[u8; 16] = b"0123456789012345";

    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    // Read the contents of the input file into a buffer
    let mut input_data: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    Ok(())
}
