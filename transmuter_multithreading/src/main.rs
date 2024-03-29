use std::env;

use error::CustomError;
use stdio_processor::StdioProcessor;
use transmute::Transmutation;

mod error;
mod stdio_processor;
mod transmute;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdio_processor = StdioProcessor::new(args, |kind, input| {
        let transmutation = Transmutation::from_str(kind).ok_or(CustomError::new(&format!(
            "The unsupported transmutation provided: {}.\nFollowing are supported: {}",
            String::from(kind),
            stringify_possible_transmutations()
        )))?;
        transmutation.transmute(input)
    });
    stdio_processor.run();
}

fn stringify_possible_transmutations() -> String {
    let transmutations = Transmutation::all_variants()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{transmutations}]")
}
