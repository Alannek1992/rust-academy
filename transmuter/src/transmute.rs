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
    Slugify => "slugify"
);

pub fn transmute(input_text: &str, kind: TransmutationKind) -> String {
    match kind {
        TransmutationKind::Lowercase => input_text.to_lowercase(),
        TransmutationKind::Uppercase => input_text.to_uppercase(),
        TransmutationKind::NoSpaces => input_text.to_lowercase(),
        TransmutationKind::Slugify => slug::slugify(input_text),
    }
}
