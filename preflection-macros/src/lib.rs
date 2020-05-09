#![warn(clippy::all)]

pub(crate) mod attr_utils;
pub(crate) mod errors;
#[cfg(feature = "has-field")]
mod has_field;
mod has_fields;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(HasFields, attributes(preflection))]
pub fn has_fields_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    match has_fields::has_fields_derive_impl(&derive_input) {
        Ok(result) => result.into(),
        Err(err) => err.into(),
    }
}

#[cfg(feature = "has-field")]
#[proc_macro_derive(HasField, attributes(preflection))]
pub fn has_field_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    match has_field::has_field_derive_impl(&derive_input) {
        Ok(output) => output.into_token_stream().into(),
        Err(err) => err.into(),
    }
}
