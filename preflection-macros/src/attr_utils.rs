use crate::errors::GetHelperAttrError;
use attribution::attr_args;
use proc_macro2::Ident;
use proc_macro2::Span;
use syn::Attribute;
use syn::Field;

pub fn get_preflection_attr(field: &Field) -> Result<HelperAttr, GetHelperAttrError> {
    // Parse all the preflection helper attributes
    let mut helper_attrs = field
        .attrs
        .iter()
        .filter(|attr| is_preflection_attr(attr))
        .map(|attr| attr.tokens.clone())
        .map(syn::parse2::<HelperAttr>)
        .collect::<Result<Vec<_>, _>>()?;

    // Ensure that there are not multiple helper attributes defined
    let second_preflection_attr_exists = helper_attrs.len() > 1;
    if second_preflection_attr_exists {
        let span = field
            .ident
            .as_ref()
            .map(Ident::span)
            .unwrap_or_else(|| Span::call_site());
        return Err(GetHelperAttrError::MultipleAttributes { span });
    }

    Ok(helper_attrs.pop().unwrap_or_default())
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
