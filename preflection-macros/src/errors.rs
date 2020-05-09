use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::Error as ParseError;
use syn::Error as SynError;

#[derive(Debug)]
pub struct PreflectionMacroError {
    message: String,
    span: Span,
}

impl PreflectionMacroError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl From<GetHelperAttrError> for PreflectionMacroError {
    fn from(src: GetHelperAttrError) -> Self {
        let message = src.to_string();
        let span = match src {
            GetHelperAttrError::ExtraTokens { span } => span,
            GetHelperAttrError::MissingGroup { span } => span,
            GetHelperAttrError::MultipleAttributes { span } => span,
            GetHelperAttrError::ParseError { parse_error } => parse_error.span(),
        };

        PreflectionMacroError::new(message, span)
    }
}

impl Into<TokenStream> for PreflectionMacroError {
    fn into(self) -> TokenStream {
        Into::<TokenStream2>::into(self).into()
    }
}

impl Into<TokenStream2> for PreflectionMacroError {
    fn into(self) -> TokenStream2 {
        SynError::new(self.span, self.message).to_compile_error()
    }
}

#[derive(Debug)]
pub enum GetHelperAttrError {
    MultipleAttributes { span: Span },
    ParseError { parse_error: syn::parse::Error },
    ExtraTokens { span: Span },
    MissingGroup { span: Span },
}

impl Display for GetHelperAttrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GetHelperAttrError::MultipleAttributes { .. } => write!(
                f,
                "Only one preflection attribute can be applied to each field."
            ),
            GetHelperAttrError::ParseError { parse_error } => write!(
                f,
                "There was a problem parsing the attribute body: {}",
                parse_error
            ),
            GetHelperAttrError::ExtraTokens { .. } => write!(f, "Found extraneous tokens."),
            GetHelperAttrError::MissingGroup { .. } => write!(
                f,
                "Expected the input to be grouped (e.g. #[preflection(ignore)]"
            ),
        }
    }
}

impl From<ParseError> for GetHelperAttrError {
    fn from(parse_error: ParseError) -> Self {
        Self::ParseError { parse_error }
    }
}
