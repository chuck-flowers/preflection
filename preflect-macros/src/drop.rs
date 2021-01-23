#![cfg(feature = "partial-drop")]

use proc_macro2::Ident;
use proc_macro2::Span as Span2;
use syn::parse_quote;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Field;
use syn::ItemImpl;
use syn::LitStr;

pub fn partial_drop_derive_impl(tagged: &DeriveInput) -> Result<ItemImpl, syn::Error> {
    let type_name = &tagged.ident;
    if let Data::Struct(tagged_struct) = &tagged.data {
        Ok(impl_partial_drop_for_struct(type_name, tagged_struct))
    } else {
        let span = Span2::call_site();
        let message = "'PartialDrop' can only be implemented for structs".to_string();
        let error = syn::Error::new(span, message);
        Err(error)
    }
}

fn impl_partial_drop_for_struct(type_name: &Ident, tagged_struct: &DataStruct) -> ItemImpl {
    let field_checks = build_field_checks(tagged_struct);
    parse_quote! {
        impl ::preflect::drop::PartialDrop for #type_name {
            unsafe fn drop_all_fields_except(&mut self, field_names: &[&'static str]) {
                #(#field_checks)*
            }
        }
    }
}

fn build_field_checks(tagged_struct: &DataStruct) -> impl Iterator<Item = syn::ExprIf> + '_ {
    tagged_struct.fields.iter().map(build_field_check)
}

fn build_field_check(field: &Field) -> syn::ExprIf {
    let field_ident = get_ident_from_field(field);
    let field_lit = get_field_ident_literal(field);
    let field_ty = &field.ty;
    parse_quote! {
        if !field_names.contains(&#field_lit) {
            let #field_ident = &mut self.#field_ident;
            if ::core::mem::needs_drop::<#field_ty>() {
                ::core::ptr::drop_in_place(#field_ident);
            }
        }
    }
}

fn get_ident_from_field(field: &Field) -> Ident {
    field.ident.as_ref().unwrap().clone()
}

fn get_field_ident_literal(field: &Field) -> LitStr {
    let field_ident = field.ident.as_ref().unwrap();
    LitStr::new(&field_ident.to_string(), field_ident.span())
}
