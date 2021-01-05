pub use preflect_macros::HasField;

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
