use crate::error::Result;
use colored::{Color, Colorize};
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
