use crate::errors::GetHelperAttrError;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenTree;
use syn::parse::Error as ParseError;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::punctuated::Punctuated;
use syn::Attribute;
use syn::Field;
use syn::Meta;
use syn::Token;

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

#[derive(Default)]
pub struct HelperAttr {
    ignore: bool,
}

impl HelperAttr {
    pub const fn ignore(&self) -> bool {
        self.ignore
    }
}

impl Parse for HelperAttr {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut attr = HelperAttr::default();
        if input.is_empty() {
            return Ok(attr);
        }

        let meta_list = Punctuated::<Meta, Token!(,)>::parse_separated_nonempty(input)?;
        for meta in meta_list {
            match meta {
                Meta::Path(path) => {
                    let ident = path.get_ident();
                    if ident.map(|i| i == "ignore").unwrap_or(false) {
                        attr.ignore = true;
                    } else {
                        return Err(ParseError::new_spanned(path, "Unexpected attribute field."));
                    }
                }
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident();
                    if ident.map(|i| i == "ignore").unwrap_or(false) {
                        if let syn::Lit::Bool(b) = nv.lit {
                            attr.ignore = b.value;
                        } else {
                            return Err(ParseError::new_spanned(
                                nv,
                                "The value of 'ignore' should be a bool.",
                            ));
                        }
                    }
                }
                other => {
                    return Err(ParseError::new_spanned(
                        other,
                        "Unrecognized attribute data.",
                    ));
                }
            }
        }

        Ok(attr)
    }
}
