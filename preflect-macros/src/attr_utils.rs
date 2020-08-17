use crate::errors::GetHelperAttrError;
use attribution::AttrArgs;
use proc_macro2::Ident;
use proc_macro2::Span;
use syn::Attribute;
use syn::Field;

pub fn get_preflect_attr(field: &Field) -> Result<HelperAttr, GetHelperAttrError> {
    // Parse all the preflect helper attributes
    let mut helper_attrs = field
        .attrs
        .iter()
        .filter(|attr| is_preflect_attr(attr))
        .map(|attr| attr.tokens.clone())
        .map(syn::parse2::<HelperAttr>)
        .collect::<Result<Vec<_>, _>>()?;

    // Ensure that there are not multiple helper attributes defined
    let second_preflect_attr_exists = helper_attrs.len() > 1;
    if second_preflect_attr_exists {
        let span = field
            .ident
            .as_ref()
            .map(Ident::span)
            .unwrap_or_else(|| Span::call_site());
        return Err(GetHelperAttrError::MultipleAttributes { span });
    }

    Ok(helper_attrs.pop().unwrap_or_default())
}

fn is_preflect_attr(attr: &Attribute) -> bool {
    attr.path
        .get_ident()
        .map(|i| i == "preflect")
        .unwrap_or(false)
}

#[derive(AttrArgs, Debug)]
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
