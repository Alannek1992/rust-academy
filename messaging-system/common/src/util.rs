use crate::{
    api::FileData,
    error::{MsgSystemError, Result},
};
use colored::{Color, Colorize};
use log::trace;
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
    trace!(
        "Creating a file: {} in : {}",
        file_data.file_name,
        output_directory
    );
    let mut file_path = Path::new(output_directory).join(&file_data.file_name);

    if let Some(extension) = &file_data.file_extension {
        file_path.set_extension(extension);
    }

    // Create the directory structure if it doesn't exist
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|_| MsgSystemError::CannotWriteFile {
            output_directory: output_directory.to_string(),
        })?;
    }

    let mut file = File::create(file_path).map_err(|_| MsgSystemError::CannotWriteFile {
        output_directory: output_directory.to_string(),
    })?;

    file.write_all(&file_data.bytes)
        .map_err(|_| MsgSystemError::CannotWriteFile {
            output_directory: output_directory.to_string(),
        })?;

    Ok(())
}

pub fn print_error_to_stdout(err: impl Error) {
    eprintln!("{}", err.to_string().red())
}

pub fn print_msg_to_stdout(msg: &str, color: ColorFacade) {
    println!("{}", msg.color(color.convert()))
}

pub fn default_error_handler(e: impl Error) -> ! {
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
