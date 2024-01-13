use std::{fmt::Display, io::{Read, Write}};

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
