//! Glob engine abstractions.
//!
//! If the `ec4rs_glob` feature is enabled,
//! this module also includes a re-export of `ec4rs_glob`.

/// A parsed glob pattern which can be used for matching paths.
///
/// This is intended to expose only the subset of functionality relevant for parsing
/// EditorConfig files, and therefore does not include any way to configure a builder
/// for the glob pattern.
pub trait Pattern {
    /// The type of error returned by a failed parse.
    type Error: std::error::Error + Sync + Send + 'static;
    /// Attempts to parse `Self` out of a string.
    fn parse(pattern: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
    /// Returns `true` if the provided path matches `Self`.
    ///
    /// If evaluation errors, such as due to depth limits being reached,
    /// this function must return `false`.
    #[must_use]
    fn matches(&self, path: &std::path::Path) -> bool;
}

#[cfg(feature = "ec4rs_glob")]
pub use ec4rs_glob::*;

#[cfg(feature = "ec4rs_glob")]
impl Pattern for Glob {
    type Error = std::convert::Infallible;

    fn parse(pattern: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        // TODO: Size and depth limits.
        Ok(Glob::new(pattern))
    }

    fn matches(&self, path: &std::path::Path) -> bool {
        self.matches(path)
    }
}

#[cfg(feature = "globset")]
impl Pattern for globset::GlobMatcher {
    type Error = globset::Error;

    fn parse(pattern: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let glob = globset::Glob::new(pattern)?;
        Ok(glob.compile_matcher())
    }

    fn matches(&self, path: &std::path::Path) -> bool {
        self.is_match(path)
    }
}
