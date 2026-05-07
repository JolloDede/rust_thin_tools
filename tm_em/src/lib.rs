extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{LitStr, parse_macro_input};

mod parse;
use parse::*;

#[proc_macro]
pub fn turing_machine(input: TokenStream) -> TokenStream {
    expand_bits(input)
}

fn expand_bits(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let bit_str = input.value();

    let goedel = match GoedelnummerInput::parse_goedel(bit_str) {
        Ok(value) => value,
        Err(err) => return err.to_compile_error().into(),
    };

    let max_state = match goedel.zustaende.iter().map(|z| z.0).max() {
        Some(value) if value > 0 => value,
        _ => {
            return syn::Error::new(input.span(), "no states parsed from Gödel input")
                .to_compile_error()
                .into();
        }
    };

    let state_variants: Vec<_> = (1..=max_state)
        .map(|index| format_ident!("Q{index}"))
        .collect();

    let mut transition_tokens = Vec::with_capacity(goedel.transitions.len());
    for transition in goedel.transitions {
        let start = state_ident(transition.start_zustand, max_state, input.span());
        let end = state_ident(transition.end_zustand, max_state, input.span());

        let read = match map_symbol(transition.lesen) {
            Ok(value) => value,
            Err(err) => return err.to_compile_error().into(),
        };

        let write = match map_symbol(transition.schreiben) {
            Ok(value) => value,
            Err(err) => return err.to_compile_error().into(),
        };

        let direction = match transition.direction {
            Direction::Left => quote! { crate::tm_emulator::Direction::Left },
            Direction::Right => quote! { crate::tm_emulator::Direction::Right },
        };

        transition_tokens.push(quote! {
            crate::tm_emulator::Transition {
                start: #start,
                read: #read,
                end: #end,
                write: #write,
                direction: #direction,
            }
        });
    }

    let expanded = quote! {
        {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub enum State {
                #(#state_variants),*
            }

            let transitions = vec![#(#transition_tokens),*];
            crate::tm_emulator::TuringMachine::new(
                transitions,
                crate::tm_emulator::Band::new(Vec::new()),
                State::Q1,
            )
        }
    };

    TokenStream::from(expanded)
}

fn state_ident(
    zustand: Zustand,
    max_state: usize,
    span: proc_macro2::Span,
) -> proc_macro2::TokenStream {
    if zustand.0 == 0 || zustand.0 > max_state {
        return syn::Error::new(span, "invalid state index in Gödel input").to_compile_error();
    }

    let ident = format_ident!("Q{}", zustand.0);
    quote! { State::#ident }
}

fn map_symbol(value: usize) -> syn::Result<proc_macro2::TokenStream> {
    let symbol = match value {
        1 => '0',
        2 => '1',
        3 => ' ',
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "symbol must be 1 (0), 2 (1), or 3 (space)",
            ));
        }
    };

    Ok(quote! { #symbol })
}
