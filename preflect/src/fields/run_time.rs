use core::any::Any;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
pub use preflect_macros::HasFields;

/// The result of accessing a field dynamically
pub type FieldAccessResult<T> = Result<T, FieldAccessError>;

/// A trait that can be used to dynamically access the fields of a struct at
/// runtime.
pub trait BaseHasFields {
    /// Gets an immutable reference to a field using the name of the field.
    fn get_field_raw<'s>(&'s self, name: &str) -> FieldAccessResult<&'s dyn Any>;

    /// Gets a mutable reference to a field using the name of the field.
    fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut dyn Any>;
}

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

/// A trait that provides useful extension methods that make dynamically
/// accessing struct fields at runtime easier.
pub trait HasFields: BaseHasFields {
    /// Gets an immutable reference to a field using the name of the field.
    fn get_field<'s, T: 'static>(&'s self, name: &str) -> FieldAccessResult<&'s T>;

    /// Gets a mutable reference to a field using the name of the field.
    fn get_field_mut<'s, T: 'static>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut T>;
}

impl<T: BaseHasFields> HasFields for T {
    fn get_field<'s, U: 'static>(&'s self, name: &str) -> FieldAccessResult<&'s U> {
        self.get_field_raw(name)?
            .downcast_ref::<U>()
            .ok_or(FieldAccessError::InvalidType)
    }

    fn get_field_mut<'s, U: 'static>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut U> {
        self.get_field_mut_raw(name)?
            .downcast_mut::<U>()
            .ok_or(FieldAccessError::InvalidType)
    }
}
