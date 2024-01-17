use std::{fmt::Display, process};

use anyhow::Error;
use colored::{Color, Colorize};

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

pub fn print_msg_to_stdout(msg: &str, color: ColorFacade) {
    println!("{}", msg.color(color.convert()))
}

pub fn print_msg_to_stderr<T: Display>(msg: T) {
    eprintln!("{}", msg.to_string().red())
}

pub fn default_error_handler(e: Error) -> ! {
    print_msg_to_stderr(e);
    process::exit(1);
}
