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

#[macro_export]
macro_rules! config_line {
    ($keyword: expr, $arg: expr) => {{
        format!("{}={}", $keyword, $arg)
    }};
    ($keyword: expr, $arg: expr, $arg2: expr) => {{
        format!("{}={}={}", $keyword, $arg, $arg2)
    }};
    ($keyword: expr, $arg: expr, $arg2: expr, $arg3: expr) => {{
        format!("{}={}={}={}", $keyword, $arg, $arg2, $arg3)
    }};
}
#[macro_export]
macro_rules! parse_config_line {
    ($line: expr) => {{
        let parts: Vec<String> = $line.split('=').map(|w| w.trim().to_string()).collect();
        if parts.len() < 2 {
            None
        } else {
            Some(parts)
        }
    }};
}