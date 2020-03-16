mod errors;

pub use self::errors::FieldAccessError;
use core::any::Any;
pub use preflection_macros::HasFields;

pub type FieldAccessResult<T> = Result<T, FieldAccessError>;

/// A trait that can be used to dynamically access the fields of a struct at
/// runtime.
pub trait HasFields {
    /// Gets an immutable reference to a field using the name of the field.
    fn get_field_raw<'s>(&'s self, name: &str) -> FieldAccessResult<&'s dyn Any>;

    /// Gets a mutable reference to a field using the name of the field.
    fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut dyn Any>;
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

#[cfg(test)]
mod tests {

    use super::*;

    struct User {
        id: u32,
    }

    impl HasFields for User {
        fn get_field_raw<'s>(&'s self, name: &str) -> FieldAccessResult<&'s dyn Any> {
            match name {
                "id" => Ok(&self.id),
                _ => Err(FieldAccessError::MissingField),
            }
        }
        fn get_field_mut_raw<'s>(&'s mut self, name: &str) -> FieldAccessResult<&'s mut dyn Any> {
            match name {
                "id" => Ok(&mut self.id),
                _ => Err(FieldAccessError::MissingField),
            }
        }
    }

    #[test]
    fn get_field_test() {
        let user = User { id: 1 };

        let id = user.get_field::<u32>("id").unwrap();

        assert_eq!(id, &user.id);
    }
}
