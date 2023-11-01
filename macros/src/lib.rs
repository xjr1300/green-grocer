use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod entity_id;
use entity_id::derive_entity_id_impl;

#[proc_macro_derive(EntityId)]
pub fn derive_entity_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_entity_id_impl(input) {
        Ok(token_stream) => TokenStream::from(token_stream),
        Err(err) => TokenStream::from(err.into_compile_error()),
    }
}
