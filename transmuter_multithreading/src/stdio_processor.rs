use std::{error::Error, io, process, thread};

use colored::Colorize;

use crate::error::Result;

pub struct StdioProcessor<F>
where
    F: Fn(&str, &str) -> Result<String> + Send + Sync,
{
    env_args: Vec<String>,
    processor: Option<F>,
}

impl<F> StdioProcessor<F>
where
    F: Fn(&str, &str) -> Result<String> + Send + Sync + 'static,
{
    pub fn new(env_args: Vec<String>, processor: F) -> StdioProcessor<F> {
        StdioProcessor {
            env_args,
            processor: Some(processor),
        }
    }

    fn read_user_input() -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn print_error(err: &str) {
        eprintln!("{}{}", "Error: ".red(), err.red());
    }

    fn print_result(result: &str) {
        println!("{}", result.green());
    }

    pub fn run(&mut self) {
        if self.env_args.len() < 2 {
            self.interactive_mode();
        } else {
            self.one_shot_mode();
        }
        process::exit(0);
    }

    fn interactive_mode(&mut self) {
        let (tx, rx) = flume::unbounded();

        let input_thread = thread::spawn(move || {
            println!("Provide the transmutation in following format: <command> <input>. Type exit for quitting the program");
            loop {
                match Self::read_user_input() {
                    Ok(line) => {
                        let input: Vec<&str> = line.split_whitespace().collect();
                        let Some((kind, text)) = input.split_first() else {
                            Self::print_error(
                                "The provided input should have following format: <command> <input>",
                            );
                            continue;
                        };

                        if *kind == "exit" {
                            // Allow users to exit the interactive mode gracefully
                            println!("Exiting the interactive mode.");
                            break;
                        }

                        if let Err(e) = tx.send((kind.to_string(), text.join(" "))) {
                            Self::print_error(&e.to_string());
                        }
                    }
                    Err(e) => {
                        Self::print_error(&e.to_string());
                        break;
                    }
                }
            }
        });

        let processing_thread = {
            let processor = self.processor.take().unwrap();
            thread::spawn(move || {
                for (kind, text) in rx.iter() {
                    match processor(&kind, &text) {
                        Ok(output) => {
                            Self::print_result(&output);
                        }
                        Err(e) => {
                            Self::print_error(&e.to_string());
                            continue;
                        }
                    }
                }
            })
        };

        let input_thread_result = input_thread.join();
        let processing_thread_result = processing_thread.join();

        if let Err(_) = input_thread_result {
            Self::print_error("Input reading thread failed");
            process::exit(1);
        }
        if let Err(_) = processing_thread_result {
            Self::print_error("Input processing thread failed");
            process::exit(1);
        }
    }

    fn one_shot_mode(&mut self) {
        let error_handler = |e: Box<dyn Error>| {
            Self::print_error(&e.to_string());
            process::exit(1)
        };
        println!("Please enter your text");
        let input = Self::read_user_input().unwrap_or_else(error_handler);
        let result = (self.processor.take().unwrap())(&self.env_args[1], &input)
            .unwrap_or_else(error_handler);
        Self::print_result(&result);
    }
}
