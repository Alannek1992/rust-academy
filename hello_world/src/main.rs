use std::fmt::{Display, Formatter, Result};

fn main() {
    println!("{}", HelloWorld);
}

struct HelloWorld;

impl Display for HelloWorld {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Hello, World!")
    }
}
