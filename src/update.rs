use crate::Error;

/// The `Update` trait of the Aragog library.
/// This trait provides the possibility to update a Type from an other one. Its main use
/// it to apply modifications from a Http form on a [`Record`] model instance.
///
/// [`Record`]: crate::Record
pub trait Update<T> {
    /// Update the `Self` field values `T`. The object takes a mutable reference of itself and is directly
    /// updated.
    ///
    /// # Errors
    ///
    /// Can fail and return an error, the error is in most of the cases an [`Error`]::[`ValidationError`]
    /// on fields validation failure
    ///
    /// [`Error`]: crate::Error
    /// [`ValidationError`]: crate::Error::ValidationError
    fn update(&mut self, form: &T) -> Result<(), Error>;

    /// Can update a mutable `value` with a new one if the `new_value` is defined (`Some`).
    /// if the `new_value` is `None` the value stays unchanged
    fn update_field_from_option<U>(value: &mut U, new_value: &Option<U>)
    where
        U: Clone,
    {
        match new_value {
            Some(val) => *value = val.clone(),
            None => (),
        };
    }
}
