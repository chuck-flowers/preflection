use crate::errors::GetHelperAttrError;
use attribution::attr_args;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenTree;
use syn::Attribute;
use syn::Field;

pub fn get_preflection_attr(field: &Field) -> Result<HelperAttr, GetHelperAttrError> {
    // All attributes on the field
    let attrs = field.attrs.iter();

    // All preflection attributes on the field
    let preflection_attrs = attrs.filter(|attr| is_preflection_attr(attr));

    // The content of the preflection attributes
    // Example: #[preflection(ignore = true)] -> preflection(ignore = true)
    let mut preflection_attr_bodies = preflection_attrs.map(|attr| attr.tokens.clone());

    // The body content of the preflection attributes
    // Example: #[preflection(ignore = true)] -> (ignore = true)
    let mut first_attr_body = preflection_attr_bodies
        .next()
        .unwrap_or_default()
        .into_iter();

    // Ensure that there is not a second preflection attribute declared
    let second_preflection_attr_exists = preflection_attr_bodies.next().is_some();
    if second_preflection_attr_exists {
        let span = field
            .ident
            .as_ref()
            .map(Ident::span)
            .unwrap_or_else(|| Span::call_site());
        return Err(GetHelperAttrError::MultipleAttributes { span });
    }

    match first_attr_body.next() {
        Some(TokenTree::Group(group)) => {
            // Ensure there is not additional data after the first group
            if let Some(extra) = first_attr_body.next() {
                Err(GetHelperAttrError::ExtraTokens { span: extra.span() })
            } else {
                Ok(syn::parse2(group.stream())?)
            }
        }
        Some(other) => Err(GetHelperAttrError::MissingGroup { span: other.span() }),
        None => Ok(HelperAttr::default()),
    }
}

fn is_preflection_attr(attr: &Attribute) -> bool {
    attr.path
        .get_ident()
        .map(|i| i == "preflection")
        .unwrap_or(false)
}

#[attr_args]
pub enum HelperAttr {
    Ignore { ignore: bool },
    Alias { alias: Vec<String> },
    Default,
}

impl HelperAttr {
    pub fn ignore(&self) -> bool {
        match self {
            HelperAttr::Ignore { ignore } => *ignore,
            _ => false,
        }
    }
}

impl Default for HelperAttr {
    fn default() -> Self {
        Self::Default
    }
}
