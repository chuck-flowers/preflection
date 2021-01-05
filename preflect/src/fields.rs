//! Provides facilities for using reflection to access fields and their values.

mod errors;

pub use self::errors::FieldAccessError;
use core::any::Any;
pub use preflect_macros::HasField;
pub use preflect_macros::HasFields;

/// The result of accessing a field dynamically
pub type FieldAccessResult<T> = Result<T, FieldAccessError>;

/// A trait that can be used to dynamically access the fields of a struct at
/// runtime.
pub trait HasFields {
    /// Gets an immutable reference to a field using the name of the field.
    fn get_field_raw<'s>(&'s self, name: &str) -> FieldAccessResult<&'s dyn Any>;

    /// Gets a mutable reference to a field using the name of the field.
    fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut dyn Any>;
}

/// A type who has a field that can be accessed through reflection.
pub trait BaseHasField<const NAME: &'static str> {
    /// The type of the field's value.
    type FieldType: 'static;

    /// Calculates the number of bytes from the pointer to the struct at which
    /// the field resides.
    fn offset() -> usize;
}

/// Represents a struct that has a field with a specific name and type.
pub trait HasField<const NAME: &'static str>: BaseHasField<NAME> {
    /// Gets an immutable reference to the field.
    fn get_field(&self) -> &Self::FieldType;

    /// Gets a mutable reference to the field.
    fn get_field_mut(&mut self) -> &mut Self::FieldType;
}

impl<T, const NAME: &'static str> HasField<NAME> for T
where
    T: BaseHasField<NAME>,
{
    fn get_field(&self) -> &Self::FieldType {
        let base_address = self as *const Self as usize;
        let ptr = (base_address + Self::offset()) as *const Self::FieldType;
        unsafe { ptr.as_ref().unwrap() }
    }

    fn get_field_mut(&mut self) -> &mut Self::FieldType {
        let base_address = self as *mut Self as usize;
        let ptr = (base_address + Self::offset()) as *mut Self::FieldType;
        unsafe { ptr.as_mut().unwrap() }
    }
}

/// A trait that provides useful extension methods that make dynamically
/// accessing struct fields at runtime easier.
pub trait HasFieldsExt: HasFields {
    /// Gets an immutable reference to a field using the name of the field.
    fn get_field<'s, T: 'static>(&'s self, name: &str) -> FieldAccessResult<&'s T>;

    /// Gets a mutable reference to a field using the name of the field.
    fn get_field_mut<'s, T: 'static>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut T>;
}

impl<T: HasFields> HasFieldsExt for T {
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
