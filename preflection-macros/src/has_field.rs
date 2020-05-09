use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::Field;
use syn::Ident;
use syn::LitStr;

pub fn has_field_derive_impl(derive_input: &DeriveInput) -> TokenStream2 {
    let ty_name = &derive_input.ident;
    if let Data::Struct(data_struct) = &derive_input.data {
        data_struct
            .fields
            .iter()
            .map(|field| field_impl(ty_name, field))
            .collect()
    } else {
        panic!("HasField can only be derived for structs.")
    }
}

fn field_impl(ty_name: &Ident, field: &Field) -> TokenStream2 {
    let field_ty = &field.ty;
    let field_ident = &field.ident.as_ref().unwrap();
    let field_name = LitStr::new(&field_ident.to_string(), field_ident.span());
    quote! {
        impl preflection::fields::HasField<#field_ty, #field_name> for #ty_name {
            fn get_field<'a>(&'a self) -> &'a #field_ty {
                &self.#field_ident
            }

            fn get_field_mut<'a>(&'a mut self) -> &'a mut #field_ty {
                &mut self.#field_ident
            }

            fn into_field(self) -> #field_ty {
                self.#field_ident
            }
        }
    }
}
