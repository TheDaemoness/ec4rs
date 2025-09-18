use crate::glob::Pattern;
use crate::string::{ParseError, ToSharedString};
use crate::Properties;

use std::path::Path;

// Glob internals aren't stable enough to safely implement PartialEq here.

/// One section of an EditorConfig file.
#[derive(Clone)]
pub struct Section<P: Pattern> {
    pattern: Result<P, ParseError<P::Error>>,
    props: crate::Properties,
}

impl<P: Pattern> Section<P> {
    /// Constructs a new [`Section`] that applies to files matching the specified pattern.
    ///
    /// If pattern parsing errors, the error will be retained internally
    /// and no paths will be considered to match the pattern. Errors can be detected with
    /// either [`or_err`][Self::or_err] or [`pattern`][Self::pattern].
    pub fn new(pattern: &str) -> Self {
        Section {
            pattern: P::parse(pattern).map_err(|error| ParseError {
                error,
                string: pattern.into(),
            }),
            props: crate::Properties::new(),
        }
    }
    /// Returns `Ok(self)` if there was no pattern parse error,
    /// otherwise returns the error.
    pub fn or_err(self) -> Result<Self, ParseError<P::Error>> {
        if let Err(e) = self.pattern {
            Err(e)
        } else {
            Ok(self)
        }
    }
    /// Returns true if and only if this section applies to a file at the specified path.
    pub fn applies_to(&self, path: impl AsRef<Path>) -> bool {
        // MSRV of 1.56 prevents use of is_ok_and from 1.70.
        match self.pattern.as_ref() {
            Ok(p) => p.matches(path.as_ref()),
            _ => false,
        }
    }
    /// Returns a reference to either the pattern or the error.
    pub fn pattern(&self) -> &Result<P, ParseError<P::Error>> {
        &self.pattern
    }
    /// Returns a shared reference to the internal [`Properties`] map.
    pub fn props(&self) -> &Properties {
        &self.props
    }
    /// Returns a mutable reference to the internal [`Properties`] map.
    pub fn props_mut(&mut self) -> &mut Properties {
        &mut self.props
    }
    /// Extracts the [`Properties`] map from `self`.
    pub fn into_props(self) -> Properties {
        self.props
    }
    /// Adds a property with the specified key, lowercasing the key.
    pub fn insert(&mut self, key: impl ToSharedString, val: impl ToSharedString) {
        self.props
            .insert_raw_for_key(key.to_shared_string().into_lowercase(), val)
    }
}

impl<P: Pattern> crate::PropertiesSource for &Section<P> {
    /// Adds this section's properties to a [`Properties`].
    ///
    /// This implementation is infallible.
    fn apply_to(
        self,
        props: &mut Properties,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error> {
        let path_ref = path.as_ref();
        if self.applies_to(path_ref) {
            let _ = self.props.apply_to(props, path_ref);
        }
        Ok(())
    }
}
