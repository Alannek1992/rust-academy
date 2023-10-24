use std::{
    env,
    error::Error,
    io::{self, Read},
    process,
};

use colored::Colorize;
use error::{CustomError, Result};
use transmute::Transmutation;

mod error;
mod transmute;
fn main() {
    let args: Vec<String> = env::args().collect();
    let transmutation = parse_transmutation(&args).unwrap_or_else(|e| handle_error(e));
    let input = read_user_input().unwrap_or_else(|e| handle_error(e));
    print_result(&input, transmutation);
}

fn parse_transmutation(args: &[String]) -> Result<Transmutation> {
    if args.len() < 2 {
        return Err(CustomError::new(&format!(
            "You have to provide transmutation kind as CLI argument.\nFollowing are supported: {}",
            stringify_possible_transmutations()
        )));
    }
    Transmutation::from_str(&args[1]).ok_or(CustomError::new(&format!(
        "The unsupported transmutation provided: {}.\nFollowing are supported: {}",
        String::from(&args[1]),
        stringify_possible_transmutations()
    )))
}

fn stringify_possible_transmutations() -> String {
    let transmutations = Transmutation::all_variants()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{transmutations}]")
}

fn read_user_input() -> Result<String> {
    println!("Please enter your text.");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input)
}

fn handle_error(err: Box<dyn Error>) -> ! {
    eprintln!("{}{}", "Error: ".red(), err.to_string().red());
    process::exit(1);
}

fn print_result(input: &str, transmutation: Transmutation) {
    println!(
        "\n{}\n",
        transmutation
            .transmute(&input)
            .unwrap_or_else(|e| handle_error(e))
            .green()
    );
}
