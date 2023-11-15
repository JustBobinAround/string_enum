use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(StringEnum)]
pub fn string_enum(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as ItemEnum);
    let enum_name = &input.ident;
    let variants = input.variants.iter().map(|variant| &variant.ident);
    let mut formated_variants = String::new();

    for variant in variants {
        formated_variants = format!("{}stringify!({})=> Some({}::{}) ,\n", formated_variants, variant, enum_name, variant);
    }

    //well this was weird
    let expanded = format!(r#"
        impl {} {{
            pub fn str_match(s: &str) -> Option<Self> {{
                match s {{
                    {}
                    _ => None,
                }}
            }}
        }}
        "#, enum_name, formated_variants);

    expanded.parse().unwrap()
}
