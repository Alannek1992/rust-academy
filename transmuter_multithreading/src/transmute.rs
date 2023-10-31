use crate::error::Result;
use crate::transmute::csv::Csv;

mod csv;

macro_rules! transmutation_enum {
    ($($variant:ident => $str:expr),*) => {

        pub enum Transmutation {
            $($variant,)*
        }

        impl Transmutation {

            pub fn from_str(text: &str) -> Option<Self> {
                match text {
                    $($str => Some(Self::$variant),)*
                    _ => None,
                }
            }

            pub fn all_variants() -> Vec<Self> {
                let mut variants = Vec::new();
                $(variants.push(Self::$variant);)*
                variants
            }

            pub fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => $str,)*
                }
                .to_string()
            }

            pub fn transmute(&self, input: &str) -> Result<String> {
                let result = match self {
                    Self::Lowercase => input.to_lowercase(),
                    Self::Uppercase => input.to_uppercase(),
                    Self::NoSpaces => input.replace(" ", ""),
                    Self::Slugify => slug::slugify(input),
                    Self::Camelcase => to_camel_case(input),
                    Self::ReverseTalk => reverse_talk(input),
                    Self::Csv => Csv::from_file(input)?.to_string(),
                };

                Ok(result)
            }
        }
    };
}

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else {
            capitalize_next = true;
        }
    }

    result
}

fn reverse_talk(input: &str) -> String {
    input
        .split_whitespace()
        .map(|w| w.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

transmutation_enum!(
    Lowercase => "lowercase",
    Uppercase => "uppercase",
    NoSpaces => "no-spaces",
    Slugify => "slugify",
    Camelcase => "camelcase",
    ReverseTalk => "reverse-talk",
    Csv => "csv"
);

#[cfg(test)]
mod tests {
    use super::Transmutation;

    #[test]
    fn lowercase_transmutation() {
        let result = Transmutation::Lowercase
            .transmute("Rust: Where 'match' is a date night for enums!")
            .unwrap();
        assert_eq!(result, "rust: where 'match' is a date night for enums!");
    }

    #[test]
    fn uppercase_transmutation() {
        let result = Transmutation::Uppercase
            .transmute("Rust: Where lifetimes outlast relationships!")
            .unwrap();
        assert_eq!(result, "RUST: WHERE LIFETIMES OUTLAST RELATIONSHIPS!");
    }

    #[test]
    fn no_space_transmutation() {
        let result = Transmutation::NoSpaces
            .transmute("Rust: Making pointers point less stressfully!")
            .unwrap();
        assert_eq!(result, "Rust:Makingpointerspointlessstressfully!");
    }

    #[test]
    fn slugify_transmutation() {
        let result = Transmutation::Slugify
            .transmute("Rust: Where code 'borrows' but never forgets!")
            .unwrap();
        assert_eq!(result, "rust-where-code-borrows-but-never-forgets");
    }

    #[test]
    fn camelcase_transmutation() {
        let result = Transmutation::Camelcase.transmute(
            "Why did the Rustacean start a band? Because they knew how to play the 'Result' harmoniously, with no 'panic' in the performance!",
        ).unwrap();
        assert_eq!(result, "WhyDidTheRustaceanStartABandBecauseTheyKnewHowToPlayTheResultHarmoniouslyWithNoPanicInThePerformance");
    }

    #[test]
    fn reverse_talk_transmutation() {
        let result = Transmutation::ReverseTalk
            .transmute("Rust: Where 'match' is the only game that never ends!")
            .unwrap();
        assert_eq!(
            result,
            ":tsuR erehW 'hctam' si eht ylno emag taht reven !sdne"
        );
    }
}
