//! Types and utilities related to unparsed EditorConfig values.

use crate::PropertyValue;

use std::borrow::Cow;

/// An unset `RawValue`.
///
/// Not all unset `&RawValues` returned by this library are referentially equal to this one.
/// This exists to provide an unset raw value for whenever a reference to one is necessary.
pub static UNSET: RawValue = RawValue(Cow::Borrowed(""));

/// An unparsed property value.
///
/// This is conceptually an optional non-empty string with some convenience methods.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct RawValue(Cow<'static, str>);

impl RawValue {
    #[must_use]
    fn detect_unset(&self) -> Option<bool> {
        if self.is_unset() {
            Some(false)
        } else if "unset".eq_ignore_ascii_case(self.0.as_ref()) {
            Some(true)
        } else {
            None
        }
    }

    /// Returns true if the value is unset.
    ///
    /// Does not handle values of "unset".
    /// See [`RawValue::filter_unset`].
    #[must_use]
    pub fn is_unset(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to.an unset `RawValue`
    /// if the value case-insensitively matches `"unset"`,
    /// otherwise returns `self`.
    #[must_use]
    pub fn filter_unset(&self) -> &Self {
        if let Some(true) = self.detect_unset() {
            &UNSET
        } else {
            self
        }
    }

    /// Changes `self` to unset
    /// if the value case-insensitively matches `"unset"`.
    pub fn filter_unset_mut(&mut self) -> &mut Self {
        if let Some(true) = self.detect_unset() {
            *self = UNSET.clone();
        }
        self
    }

    /// Converts this `RawValue` into a [`Result`].
    ///
    /// This function filters out values of "unset".
    /// The `bool` in the `Err` variant will be false
    /// if and only if the value was not set.
    pub fn into_result(&self) -> Result<&str, bool> {
        if let Some(v) = self.detect_unset() {
            Err(v)
        } else {
            Ok(self.0.as_ref())
        }
    }

    /// Converts this `RawValue` into an [`Option`].
    pub fn into_option(&self) -> Option<&str> {
        Some(self.0.as_ref()).filter(|v| !v.is_empty())
    }

    /// Converts this `RawValue` into `&str`.
    ///
    /// If the value was not set, returns "unset".
    #[must_use]
    pub fn into_str(&self) -> &str {
        if self.is_unset() {
            "unset"
        } else {
            self.0.as_ref()
        }
    }

    /// Sets the contained string value.
    pub fn set<T: Into<RawValue>>(&mut self, val: T) -> &mut Self {
        *self = val.into();
        self
    }

    /// Attempts to parse the contained value.
    ///
    /// If the value is unset, returns `Err(None)`.
    pub fn parse<T: PropertyValue>(&self) -> Result<T, Option<T::Err>> {
        let this = if T::MAYBE_UNSET {
            self.filter_unset()
        } else {
            self
        };
        if this.is_unset() {
            Err(None)
        } else {
            T::parse(this).map_err(Some)
        }
    }
}

impl From<String> for RawValue {
    fn from(value: String) -> Self {
        RawValue(Cow::Owned(value))
    }
}

impl From<&'static str> for RawValue {
    fn from(value: &'static str) -> Self {
        if value.is_empty() {
            UNSET.clone()
        } else {
            RawValue(Cow::Borrowed(value))
        }
    }
}
