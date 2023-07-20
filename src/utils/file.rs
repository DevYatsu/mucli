#[macro_export]
macro_rules! file {
    ($name: expr) => {{
        use std::io::Read;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($name)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        (file, buffer)
    }};
}
#[macro_export]
macro_rules! file_as_bytes {
    ($name: expr) => {{
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($name)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        (file, buffer)
    }};
}
#[macro_export]
macro_rules! file_truncate {
    ($name: expr) => {{
        use std::io::Read;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open($name)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        (file, buffer)
    }};
}
