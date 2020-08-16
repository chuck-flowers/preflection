use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

/// An error that occurred while accessing a field through the preflect API.
#[derive(Debug, Eq, PartialEq)]
pub enum FieldAccessError {
    MissingField,
    InvalidType,
}

impl Display for FieldAccessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FieldAccessError::MissingField => {
                write!(f, "The specified field did not exist for the object.")
            }
            FieldAccessError::InvalidType => {
                write!(f, "The specified field is of a different type.")
            }
        }
    }
}
