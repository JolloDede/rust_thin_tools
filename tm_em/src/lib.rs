extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    LitInt, LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

mod parse;
use parse::*;

#[proc_macro]
pub fn turing_machine(input: TokenStream) -> TokenStream {
    expand_bits(input)
}

fn expand_bits(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let bit_str = input.value();

    let goedel = GoedelnummerInput::parse_goedel(bit_str);

    let expanded = quote! {
        // vec![#(#bits),*]
    };

    TokenStream::from(expanded)
}

enum Direction {
    Left,
    Right,
}
