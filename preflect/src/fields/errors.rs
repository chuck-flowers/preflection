use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

/// An error that occurred while accessing a field through the preflect API.
#[derive(Debug, Eq, PartialEq)]
pub enum FieldAccessError {
    /// A specified field was not found for the type.
    MissingField,
    /// The type of the field was not of the expected type.
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
