use crate::attr_utils::get_preflect_attr;
use crate::errors::PreflectMacroError;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote;
use syn::Data;
use syn::DeriveInput;
use syn::Field;
use syn::Ident;
use syn::ItemImpl;
use syn::LitStr;

pub fn has_field_derive_impl(
    derive_input: &DeriveInput,
) -> Result<TokenStream, PreflectMacroError> {
    let ty_name = &derive_input.ident;
    if let Data::Struct(data_struct) = &derive_input.data {
        let mut token_stream = TokenStream::new();
        for field in data_struct.fields.iter() {
            let attr = get_preflect_attr(field)?;
            if !attr.ignore() {
                token_stream.extend(field_impl(ty_name, field).into_token_stream());
            }
        }

        Ok(token_stream)
    } else {
        let message = "".into();
        let span = ty_name.span();
        Err(PreflectMacroError::new(message, span))
    }
}

fn field_impl(ty_name: &Ident, field: &Field) -> ItemImpl {
    let field_ty = &field.ty;
    let field_ident = &field.ident.as_ref().unwrap();
    let field_name = LitStr::new(&field_ident.to_string(), field_ident.span());
    parse_quote! {
        impl ::preflect::fields::BaseHasField<#field_name> for #ty_name {
            type FieldType = #field_ty;

            fn offset() -> usize {
                ::preflect::memoffset::offset_of!(#ty_name, #field_ident)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proc_macro2::Span;

    #[test]
    fn field_impl_test() {
        let ty_name = Ident::new("User", Span::call_site());
        let field = make_field();

        let actual = field_impl(&ty_name, &field);
        let expected = parse_quote! {
            impl ::preflect::fields::BaseHasField<"id"> for User {
                type FieldType = u32;

                fn offset() -> usize {
                    ::preflect::memoffset::offset_of!(User, id)
                }
            }
        };

        assert_eq!(actual, expected)
    }

    fn make_field() -> Field {
        Field {
            attrs: vec![],
            colon_token: Some(syn::token::Colon::default()),
            ident: parse_quote!(id),
            ty: parse_quote!(u32),
            vis: parse_quote!(pub),
        }
    }
}
