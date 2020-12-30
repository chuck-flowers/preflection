use crate::attr_utils::get_preflect_attr;
use crate::errors::GetHelperAttrError;
use crate::errors::PreflectMacroError;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Error as ParseError;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::Arm;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::ExprMatch;
use syn::Field;
use syn::Fields;
use syn::Ident;
use syn::ItemImpl;
use syn::LitStr;
use syn::Token;

pub fn has_fields_derive_impl(
    derive_input: &DeriveInput,
) -> Result<TokenStream, PreflectMacroError> {
    if let Data::Struct(data_struct) = &derive_input.data {
        let struct_ident = &derive_input.ident;

        impl_has_fields_for_data_struct(struct_ident, data_struct)
            .map(ToTokens::into_token_stream)
            .map_err(PreflectMacroError::from)
    } else {
        panic!("HasFields can only be derived for structs")
    }
}

fn impl_has_fields_for_data_struct(
    struct_ident: &Ident,
    data_struct: &DataStruct,
) -> Result<ItemImpl, GetHelperAttrError> {
    let empty_punct = Punctuated::default();
    let fields = match &data_struct.fields {
        Fields::Named(fields_named) => &fields_named.named,
        Fields::Unnamed(fields_unnamed) => &fields_unnamed.unnamed,
        Fields::Unit => &empty_punct,
    };

    let reg_match = make_match(fields.iter(), false)?;
    let mut_match = make_match(fields.iter(), true)?;

    // Builds the match arms for the immutable and mutable version of the get_field method.
    Ok(parse_quote! {
        impl preflect::fields::HasFields for #struct_ident {
            fn get_field_raw<'s>(&'s self, name: &str) -> preflect::fields::FieldAccessResult<&'s dyn core::any::Any> {
                #reg_match
            }

            fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> preflect::fields::FieldAccessResult<&'s mut dyn core::any::Any> {
                #mut_match
            }
        }
    })
}

fn make_match<'a>(
    fields: impl Iterator<Item = &'a Field>,
    is_mut: bool,
) -> Result<ExprMatch, ParseError> {
    // Build a mut token if needed
    let mut_token: Option<Token![mut]> = if is_mut {
        Some(Token!(mut)(Span::call_site()))
    } else {
        None
    };

    // Build a token stream for each match arm
    let match_arms = fields
        .filter_map(|field| match make_match_arm(field, mut_token) {
            Ok(Some(arm)) => Some(Ok(arm)),
            Err(e) => Some(Err(e)),
            _ => None,
        })
        .collect::<Result<Vec<Arm>, _>>()
        .map_err(<GetHelperAttrError as Into<syn::Error>>::into)?;

    let match_statement = parse_quote! {
        match name {
            #(#match_arms,)*
            _ => core::result::Result::Err(preflect::fields::FieldAccessError::MissingField)
        }
    };

    Ok(match_statement)
}

fn make_match_arm(
    field: &Field,
    mut_token: Option<Token![mut]>,
) -> Result<Option<Arm>, GetHelperAttrError> {
    get_preflect_attr(field).map(|attr| {
        if attr.ignore() {
            None
        } else {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name_lit = LitStr::new(&field_ident.to_string(), field_ident.span());
            let arm: Arm = parse_quote! { #field_name_lit => core::result::Result::Ok(& #mut_token self.#field_ident) };
            Some(arm)
        }
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use proc_macro2::Span;
    use syn::parse_quote;
    use syn::punctuated::Punctuated;
    use syn::token::Brace;
    use syn::Arm;
    use syn::DataStruct;
    use syn::Field;
    use syn::Fields;
    use syn::FieldsNamed;

    #[test]
    fn impl_has_fields_for_data_struct_test() {
        let struct_ident = Ident::new("User", Span::call_site());
        let data_struct = make_data_struct();
        let actual = impl_has_fields_for_data_struct(&struct_ident, &data_struct).unwrap();
        let expected: ItemImpl = parse_quote! {
            impl preflect::fields::HasFields for User {
                fn get_field_raw<'s>(&'s self, name: &str) -> preflect::fields::FieldAccessResult<&'s dyn core::any::Any> {
                    match name {
                        "id" => core::result::Result::Ok(&self.id),
                        _ => core::result::Result::Err(preflect::fields::FieldAccessError::MissingField)
                    }
                }

                fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> preflect::fields::FieldAccessResult<&'s mut dyn core::any::Any> {
                    match name {
                        "id" => core::result::Result::Ok(&mut self.id),
                        _ => core::result::Result::Err(preflect::fields::FieldAccessError::MissingField)
                    }
                }
            }
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn make_match_test() {
        let fields = make_named_fields();
        let actual = make_match(fields.iter(), false).unwrap();
        let expected: ExprMatch = parse_quote! {
            match name {
                "id" => core::result::Result::Ok(&self.id),
                _ => core::result::Result::Err(preflect::fields::FieldAccessError::MissingField)
            }
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn make_match_arm_test() {
        let field = make_field();

        let actual = make_match_arm(&field, None).unwrap().unwrap();
        let expected: Arm = parse_quote! {
            "id" => core::result::Result::Ok(&self.id)
        };

        assert_eq!(actual, expected)
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
            colon_token: Some(syn::token::Colon::default()),
            ident: Some(Ident::new("id", Span::call_site())),
            ty: parse_quote!(u32),
            vis: parse_quote!(pub),
        }
    }
}
