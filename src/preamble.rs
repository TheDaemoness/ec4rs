//! [`Preamble`] and related utilities.
//!
//! The preamble's semantics are relatively undefined by the specification.
//! The one standard key-value pair that is defined for it is `root`,
//! with the other key-value pairs being required to appear in sections.
//! `ec4rs` assumes that the prelude shall only contain key-value pairs that affect the
//! processing of EditorConfig files in general (as `root` does)
//! and NOT the configuration of editors.
//!
//! The `Preamble` type exists to cover cases where `ec4rs` may need to improve preamble support
//! in the future without breaking backward compatibility.
//! At this time, it only parses and retains the `root` property.

/// A parsed EditorConfig preamble.
///
/// See the [module level documentation][crate::preamble] for more information.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Preamble {
    root: bool,
}

impl Preamble {
    /// Returns a new default [`Preamble`].
    pub fn new() -> Self {
        Preamble { root: false }
    }

    /// Returns `true` if `self` the `root` property was specified and has a value
    /// case-insensitively equal to `"true"`.
    pub fn is_root(&self) -> bool {
        self.root
    }

    /// Returns `self` with the value of the `root` property set to the provided value.
    ///
    /// EditorConfig does not meaningfully distingush between `root` not being set and
    /// `root` being set to `"false"`.
    pub fn with_root(mut self, value: bool) -> Self {
        self.root = value;
        self
    }
}
