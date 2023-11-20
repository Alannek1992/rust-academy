use crate::{api::FileData, error::Result};
use colored::{Color, Colorize};
use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    path::Path,
    process,
};

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

pub fn write_to_file(output_directory: &str, file_data: &FileData) -> Result<()> {
    let mut file_path = Path::new(output_directory).join(&file_data.file_name);

    if let Some(extension) = &file_data.file_extension {
        file_path.set_extension(extension);
    }

    // Create the directory structure if it doesn't exist
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;

    file.write_all(&file_data.bytes)?;

    Ok(())
}

pub fn print_error_to_stdout(err: Box<dyn Error>) {
    eprintln!("{}", err.to_string().red())
}

pub fn print_msg_to_stdout(msg: &str, color: ColorFacade) {
    println!("{}", msg.color(color.convert()))
}

pub fn default_error_handler(e: Box<dyn Error>) -> ! {
    print_error_to_stdout(e);
    process::exit(1);
}

pub fn is_stream_closed(stream: &mut TcpStream) -> bool {
    let mut buffer = [0; 0]; // Zero-sized buffer

    match stream.read(&mut buffer) {
        Ok(0) => true,  // 0 bytes read means the stream is closed
        Ok(_) => false, // Some bytes were read, the stream is still open
        Err(_) => true, // An error occurred, consider the stream closed
    }
}
