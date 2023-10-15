macro_rules! transmutation_enum {
    ($($variant:ident => $str:expr),*) => {

        pub enum TransmutationKind {
            $($variant,)*
        }

        impl TransmutationKind {

            pub fn from_str(text: &str) -> Option<Self> {
                match text {
                    $($str => Some(Self::$variant),)*
                    _ => None,
                }
            }

            pub fn all_variants() -> Vec<Self> {
                let mut variants = Vec::new();
                $(variants.push(TransmutationKind::$variant);)*
                variants
            }

            pub fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => $str,)*
                }
                .to_string()
            }
        }
    };
}

transmutation_enum!(
    Lowercase => "lowercase",
    Uppercase => "uppercase",
    NoSpaces => "no-spaces",
    Slugify => "slugify",
    Camelcase => "camelcase",
    ReverseTalk => "reverse-talk"
);

pub fn transmute(input_text: &str, kind: TransmutationKind) -> String {
    match kind {
        TransmutationKind::Lowercase => input_text.to_lowercase(),
        TransmutationKind::Uppercase => input_text.to_uppercase(),
        TransmutationKind::NoSpaces => input_text.replace(" ", ""),
        TransmutationKind::Slugify => slug::slugify(input_text),
        TransmutationKind::Camelcase => to_camel_case(input_text),
        TransmutationKind::ReverseTalk => reverse_talk(input_text),
    }
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

#[cfg(test)]
mod tests {
    use super::{transmute, TransmutationKind};

    #[test]
    fn lowercase_transmutation() {
        let result = transmute(
            "Rust: Where 'match' is a date night for enums!",
            TransmutationKind::Lowercase,
        );
        assert_eq!(result, "rust: where 'match' is a date night for enums!");
    }

    #[test]
    fn uppercase_transmutation() {
        let result = transmute(
            "Rust: Where lifetimes outlast relationships!",
            TransmutationKind::Uppercase,
        );
        assert_eq!(result, "RUST: WHERE LIFETIMES OUTLAST RELATIONSHIPS!");
    }

    #[test]
    fn no_space_transmutation() {
        let result = transmute(
            "Rust: Making pointers point less stressfully!",
            TransmutationKind::NoSpaces,
        );
        assert_eq!(result, "Rust:Makingpointerspointlessstressfully!");
    }

    #[test]
    fn slugify_transmutation() {
        let result = transmute(
            "Rust: Where code 'borrows' but never forgets!",
            TransmutationKind::Slugify,
        );
        assert_eq!(result, "rust-where-code-borrows-but-never-forgets");
    }

    #[test]
    fn camelcase_transmutation() {
        let result = transmute(
            "Why did the Rustacean start a band? Because they knew how to play the 'Result' harmoniously, with no 'panic' in the performance!",
            TransmutationKind::Camelcase,
        );
        assert_eq!(result, "WhyDidTheRustaceanStartABandBecauseTheyKnewHowToPlayTheResultHarmoniouslyWithNoPanicInThePerformance");
    }

    #[test]
    fn reverse_talk_transmutation() {
        let result = transmute(
            "Rust: Where 'match' is the only game that never ends!",
            TransmutationKind::ReverseTalk,
        );
        assert_eq!(
            result,
            ":tsuR erehW 'hctam' si eht ylno emag taht reven !sdne"
        );
    }
}
