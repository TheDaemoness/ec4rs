use crate::string::SharedString;

/// Trait for types that be converted to [`SharedString`]s
/// and parsed back out of the returned [`SharedString`]s.
pub trait PropertyValue:
    Sized + std::str::FromStr + crate::string::ToSharedString + Default
{
    /// Parses a value from a [`SharedString`].
    ///
    /// Some types may contain a copy of the string they were parsed from.
    /// This function allows an more-efficient implementation.
    ///
    /// For consistency reasons, if `self` contains a `SharedString`,
    /// it should not have a source.
    /// See [`SharedString::clear_source`].
    fn from_shared_string(value: &SharedString) -> Result<Self, Self::Err> {
        std::str::FromStr::from_str(value)
    }
}

/// Trait for types that are associated with property names.
///
/// Types that implement this trait will usually also implement [`PropertyValue`].
pub trait PropertyKey {
    /// The lowercase string key for this property.
    ///
    /// Used to look up the value in a [`crate::Properties`] map.
    fn key() -> &'static str;
}

/// Tests if the result of parsing the result of a `ToSharedString` conversion
/// is *not unequal* to the original value.
///
/// # Panics
/// Panics if the initial and result values are not equal.
#[cfg(test)]
pub fn test_reparse<T, E: std::fmt::Debug>(initial: &T)
where
    T: Clone + PropertyValue<Err = E> + std::fmt::Debug + PartialEq,
{
    let written: SharedString = initial.clone().to_shared_string();
    let result = T::from_shared_string(&written).expect("reparse errored");
    assert!(
        !result.ne(initial),
        "reparsed value is unequal to original; expected `{:?}`, got `{:?}`",
        initial,
        result
    )
}
