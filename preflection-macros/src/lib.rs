#![warn(clippy::all)]

#[cfg(feature = "has-field")]
mod has_field;
mod has_fields;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(HasFields)]
pub fn has_fields_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    has_fields::has_fields_derive_impl(&derive_input).into()
}

#[cfg(feature = "has-field")]
#[proc_macro_derive(HasField)]
pub fn has_field_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    has_field::has_field_derive_impl(&derive_input).into()
}
