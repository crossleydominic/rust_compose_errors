extern crate proc_macro;
use proc_macro::TokenStream;

mod either;
mod parse;
mod emit;
//use proc_macro::TokenTree::{Group, Ident, Punct, Literal};

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    let ts: TokenStream = "fn answer() -> u32 { 42 }".parse().unwrap();
    return ts;
}

#[proc_macro_attribute]
pub fn compose_errors_fn(args: TokenStream, items: TokenStream) -> TokenStream {
    //This attribute can only be used in the same places that the `derive`
    //keyword can be used so we can cheat and parse the token stream
    //as a DeriveInput.
    let parsed_items = parse_macro_input!(items as DeriveInput);
    let parsed_args = parse_macro_input!(args as parse::ComposeErrorsAttribute);

    eprintln!("ARGS: {:?}", parsed_items);
    return emit::emit_composed_errors(parsed_args, parsed_items);
}

