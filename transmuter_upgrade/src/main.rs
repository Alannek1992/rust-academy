use std::{
    env,
    io::{self, Read},
    process,
};

use error::{Error, Result};
use transmute::{transmute, TransmutationKind};

mod error;
mod transmute;
fn main() {
    let args: Vec<String> = env::args().collect();
    let transmutation_kind = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    let input = {
        println!("Please enter your text.");
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Unable to read from stdin");
        input
    };
    let output = transmute(&input, transmutation_kind).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    println!("\n{output}");
}

fn parse_args(args: &[String]) -> Result<TransmutationKind> {
    if args.len() < 2 {
        return Err(Error::new(&format!(
            "You have to provide transmutation kind as CLI argument.\nFollowing are supported: {}",
            stringify_possible_transmutations()
        )));
    }
    TransmutationKind::from_str(&args[1]).ok_or(Error::new(&format!(
        "The unsupported transmutation provided: {}.\nFollowing are supported: {}",
        String::from(&args[1]),
        stringify_possible_transmutations()
    )))
}

fn stringify_possible_transmutations() -> String {
    let transmutations = TransmutationKind::all_variants()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{transmutations}]")
}
