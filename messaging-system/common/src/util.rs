use crate::error::Result;
use colored::{Color, Colorize};
use regex::Regex;
use std::{error::Error, fs::File, io::Read};

pub enum ColorFacade {
    Yellow,
    Green,
}

impl ColorFacade {
    fn convert(&self) -> Color {
        match self {
            Self::Yellow => Color::Yellow,
            Self::Green => Color::Green,
        }
    }
}

pub fn read_from_file_path(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len() as usize;

    let mut buffer = Vec::with_capacity(file_size);

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn print_error_to_stdout(err: Box<dyn Error>) {
    eprintln!("{}", err.to_string().red())
}

pub fn print_msg_to_stdout(msg: &str, color: ColorFacade) {
    println!("{}", msg.color(color.convert()))
}

pub fn is_valid_hostname(hostname: &str) -> bool {
    // Max length of 255 characters
    if hostname.len() > 255 {
        return false;
    }

    // Character set: alphanumeric, hyphen, dot
    let regex =
        Regex::new(r"^([a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();

    regex.is_match(hostname)
}

pub fn is_port_valid(port: &str) -> bool {
    // Attempt to parse the string to a u16
    if let Ok(port) = port.parse::<u16>() {
        // Check if the port is within the valid range (0 to 65535)
        // Dont have to check the upper limit since u16 upper range is 65535
        if port > 0 {
            // You can add additional checks here if needed
            return true;
        }
    }
    false
}
