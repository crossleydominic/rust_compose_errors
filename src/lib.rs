extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod emit;
mod parse;

#[proc_macro_attribute]
pub fn compose_errors_fn(args: TokenStream, items: TokenStream) -> TokenStream {
    let parsed_items = parse_macro_input!(items as parse::DeclEnum);
    let parsed_args = parse_macro_input!(args as parse::ComposeErrorsAttribute);

    return emit::emit_composed_errors(parsed_args, parsed_items);
}
