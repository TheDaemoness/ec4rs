use crate::glob::Glob;
use crate::{rawvalue::RawValue, Properties};

use std::path::Path;

/// One section of an EditorConfig file.
pub struct Section {
    pattern: Glob,
    props: crate::Properties,
}

impl Section {
    /// Constrcts a new Section that applies to files matching the specified pattern.
    pub fn new(pattern: &str) -> Section {
        Section {
            pattern: Glob::new(pattern),
            props: crate::Properties::new(),
        }
    }
    /// Returns an immutable reference to the internal [Properties] map.
    pub fn props(&self) -> &Properties {
        &self.props
    }
    /// Adds a property to this section.
    ///
    /// ASCII-lowercases the key, but not the value.
    pub fn insert(&mut self, key: impl AsRef<str>, val: impl Into<RawValue>) {
        self.props
            .insert_raw_for_key(key.as_ref().to_lowercase(), val);
    }
    /// Returns true if and only if this section applies to a file at the specified path.
    #[must_use]
    pub fn applies_to(&self, path: impl AsRef<Path>) -> bool {
        self.pattern.matches(path.as_ref())
    }
}

impl crate::PropertiesSource for &Section {
    /// Adds this section's properties to a [Properties].
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
