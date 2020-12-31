#![cfg(partial_drop)]

/// A trait that allows portions of a struct to be dropped.
pub trait PartialDrop {
    /// Drops all fields within the struct except for those provided to the
    /// function.
    ///
    /// # Safety
    /// It is the responsibility of the caller to ensure the preserved fields
    /// are dropped as well and that the fields which were dropped are not
    /// dropped again.
    unsafe fn drop_all_fields_except(&self, field_names: &[&'static str]);
}
