use std::{error::Error, io};

use colored::Colorize;

use crate::error::Result;

pub struct StdioProcessor<F>
where
    F: Fn(&str, &str) -> Result<String>,
{
    is_multithreaded: bool,
    processor: F,
}

impl<F> StdioProcessor<F>
where
    F: Fn(&str, &str) -> Result<String>,
{
    pub fn new(is_multithreaded: bool, processor: F) -> StdioProcessor<F> {
        StdioProcessor {
            is_multithreaded,
            processor,
        }
    }

    fn read_user_input() -> Result<String> {
        println!("Please enter your text.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn handle_error(err: Box<dyn Error>) {
        eprintln!("{}{}", "Error: ".red(), err.to_string().red());
    }

    fn print_result(result: &str) {
        println!("\n{}\n", result.green());
    }

    pub fn run(&self) {
        println!("It works");
    }
}
