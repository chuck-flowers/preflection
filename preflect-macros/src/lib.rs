#![warn(clippy::all)]

mod attr_utils;
mod drop;
mod errors;
mod has_field;
mod has_fields;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(HasFields, attributes(preflect))]
pub fn has_fields_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    match self::has_fields::has_fields_derive_impl(&derive_input) {
        Ok(result) => result.into(),
        Err(err) => err.into(),
    }
}

#[proc_macro_derive(HasField, attributes(preflect))]
pub fn has_field_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    match self::has_field::has_field_derive_impl(&derive_input) {
        Ok(output) => output.into(),
        Err(err) => err.into(),
    }
}

#[proc_macro_derive(PartialDrop)]
pub fn partial_drop_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    match self::drop::partial_drop_derive_impl(&derive_input) {
        Ok(output) => output.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
