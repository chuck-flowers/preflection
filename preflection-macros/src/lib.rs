#![warn(clippy::all)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::Ident;
use syn::LitStr;
use syn::Token;

#[proc_macro_derive(HasFields)]
pub fn has_fields_derive(input_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input_stream as DeriveInput);
    has_fields_derive_impl(derive_input).into()
}

fn has_fields_derive_impl(derive_input: DeriveInput) -> TokenStream2 {
    if let Data::Struct(data_struct) = derive_input.data {
        let struct_ident = &derive_input.ident;
        impl_has_fields_for_data_struct(struct_ident, data_struct)
    } else {
        panic!("HasFields can only be derived for structs")
    }
}

fn impl_has_fields_for_data_struct(struct_ident: &Ident, data_struct: DataStruct) -> TokenStream2 {
    let fields = match data_struct.fields {
        Fields::Named(fields_named) => fields_named.named,
        Fields::Unnamed(fields_unnamed) => fields_unnamed.unnamed,
        Fields::Unit => Punctuated::default(),
    };

    let match_arms = make_match_arms(fields.iter(), false);
    let mut_match_arms = make_match_arms(fields.iter(), true);

    quote! {
        impl preflection::fields::HasFields for #struct_ident {
            fn get_field_raw<'s>(&'s self, name: &str) -> preflection::fields::FieldAccessResult<&'s dyn core::any::Any> {
                match name {
                    #match_arms
                    _ => core::result::Result::Err(preflection::fields::FieldAccessError::MissingField)
                }
            }

            fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> preflection::fields::FieldAccessResult<&'s mut dyn core::any::Any> {
                match name {
                    #mut_match_arms
                    _ => core::result::Result::Err(preflection::fields::FieldAccessError::MissingField)
                }
            }
        }
    }
}

fn make_match_arms<'a>(fields: impl Iterator<Item = &'a Field>, is_mut: bool) -> TokenStream2 {
    // Build a mut token if needed
    let mut_token: Option<Token![mut]> = if is_mut {
        Some(Token!(mut)(Span::call_site()))
    } else {
        None
    };

    // Build a token stream for each match arm
    let match_arms = fields.map(|field| make_match_arm(field, mut_token));

    // Combine the token streams into a single token stream
    quote! {
        #(#match_arms,)*
    }
}

fn make_match_arm(field: &Field, mut_token: Option<Token![mut]>) -> TokenStream2 {
    let field_ident = field.ident.as_ref().unwrap();
    let field_name_lit = LitStr::new(&field_ident.to_string(), field_ident.span());

    quote! { #field_name_lit => core::result::Result::Ok(& #mut_token self.#field_ident) }
}

#[cfg(test)]
mod tests {

    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use syn::parse_quote;
    use syn::punctuated::Punctuated;
    use syn::token::Brace;
    use syn::Data;
    use syn::DataStruct;
    use syn::DeriveInput;
    use syn::Field;
    use syn::Fields;
    use syn::FieldsNamed;
    use syn::Generics;

    #[test]
    fn impl_has_fields_for_data_struct_test() {
        let struct_ident = Ident::new("User", Span::call_site());
        let data_struct = make_data_struct();
        let actual = impl_has_fields_for_data_struct(&struct_ident, data_struct);
        let expected = quote! {
            impl preflection::fields::HasFields for User {
                fn get_field_raw<'s>(&'s self, name: &str) -> preflection::fields::FieldAccessResult<&'s dyn core::any::Any> {
                    match name {
                        "id" => core::result::Result::Ok(&self.id),
                        _ => core::result::Result::Err(preflection::fields::FieldAccessError::MissingField)
                    }
                }

                fn get_field_mut_raw<'s>(&'s self, name: &str) -> preflection::fields::FieldAccessResult<&'s dyn core::any::Any> {
                    match name {
                        "id" => core::result::Result::Ok(&mut self.id),
                        _ => core::result::Result::Err(preflection::fields::FieldAccessError::MissingField)
                    }
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string())
    }

    #[test]
    fn make_match_arms_test() {
        let fields = make_named_fields();
        let actual = make_match_arms(fields.iter(), false);
        let expected = quote! {
            "id" => core::result::Result::Ok(&self.id),
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn make_match_arm_test() {
        let field = make_field();

        let actual = make_match_arm(&field, None);
        let expected = quote! {
            "id" => core::result::Result::Ok(&self.id)
        };

        assert_eq!(actual.to_string(), expected.to_string())
    }

    fn make_derive_input() -> DeriveInput {
        DeriveInput {
            attrs: vec![],
            data: Data::Struct(make_data_struct()),
            generics: Generics::default(),
            ident: parse_quote!(User),
            vis: parse_quote!(pub),
        }
    }

    fn make_data_struct() -> DataStruct {
        DataStruct {
            fields: make_named_fields(),
            semi_token: None,
            struct_token: parse_quote!(struct),
        }
    }

    fn make_named_fields() -> Fields {
        let brace_token = Brace::default();
        let mut named = Punctuated::default();
        named.push(make_field());

        Fields::Named(FieldsNamed { brace_token, named })
    }

    fn make_field() -> Field {
        Field {
            attrs: vec![],
            colon_token: Some(parse_quote!(:)),
            ident: parse_quote!(id),
            ty: parse_quote!(u32),
            vis: parse_quote!(pub),
        }
    }
}
