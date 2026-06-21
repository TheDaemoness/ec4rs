//! The `SharedString` type and supporting types.

mod convert;
mod lowercase;

pub use convert::*;
pub(crate) use lowercase::into_lowercase;

use std::borrow::Cow;
use std::path::Path;

// Shared is a purely internal type alias.
// Its usage requires it to implement From<T> and Deref<Target = T>.

pub(crate) type Shared<T> = std::sync::Arc<T>;

// TODO: Eventually add support for an Arc-like type that uses a thin pointer here.
// Probably not triomphe::ThinArc, since we'd need to use unsafe to use it.

#[derive(Clone, Debug)]
enum SharedStringInner {
    Static(&'static str),
    Owned(Shared<str>),
}

impl SharedStringInner {
    #[inline]
    pub fn new(string: &str) -> Self {
        SharedStringInner::Owned(Shared::from(string))
    }
    #[inline]
    pub fn get(&self) -> &str {
        match self {
            SharedStringInner::Static(v) => v,
            SharedStringInner::Owned(v) => Shared::as_ref(v),
        }
    }
}

impl Default for SharedStringInner {
    fn default() -> Self {
        SharedStringInner::Static("")
    }
}

impl std::ops::Deref for SharedStringInner {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

/// A shared immutable string type.
///
/// Internally, this is either a `&'static str` or an atomically ref-counted `str`.
/// It's meant to represent either keys or values with minimal allocations
/// or duplication of data in memory, in exchange for not supporting mutation
/// or even in-place slicing.
///
/// With the `track-source` feature,
/// objects of this type can also track the file and line number they originate from.
#[derive(Clone, Debug)]
pub struct SharedString {
    value: SharedStringInner,
    #[cfg(feature = "track-source")]
    source: Option<Source>,
}

/// A `SharedString` equal to `"unset"`.
pub static UNSET: SharedString = SharedString {
    value: SharedStringInner::Static("unset"),
    #[cfg(feature = "track-source")]
    source: None,
};

/// A `SharedString` equal to `""` (an empty string).
pub static EMPTY: SharedString = SharedString {
    value: SharedStringInner::Static(""),
    #[cfg(feature = "track-source")]
    source: None,
};

// Manual-impl (Partial)Eq, (Partial)Ord, and Hash so that the source isn't considered.

impl PartialEq for SharedString {
    fn eq(&self, other: &Self) -> bool {
        *self.value == *other.value
    }
}
impl Eq for SharedString {}
impl PartialOrd for SharedString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SharedString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl std::hash::Hash for SharedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.get().hash(state);
    }
}

impl std::borrow::Borrow<str> for SharedString {
    fn borrow(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for SharedString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
impl AsRef<[u8]> for SharedString {
    fn as_ref(&self) -> &[u8] {
        self.value.as_bytes()
    }
}

impl std::ops::Deref for SharedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for SharedString {
    fn default() -> Self {
        EMPTY.clone()
    }
}

impl Default for &SharedString {
    fn default() -> Self {
        &EMPTY
    }
}

impl SharedString {
    /// Creates `Self` from the provided string.
    ///
    /// This function copies the string. If the string is `'static`, consider
    /// using [`SharedString::new_static`] instead.
    pub fn new(value: impl AsRef<str>) -> Self {
        SharedString {
            value: SharedStringInner::new(value.as_ref()),
            #[cfg(feature = "track-source")]
            source: None,
        }
    }
    /// As [`SharedString::new`] but only accepts a `&'static str`.
    ///
    /// This function does not copy the string.
    #[must_use]
    pub const fn new_static(value: &'static str) -> Self {
        SharedString {
            value: SharedStringInner::Static(value),
            #[cfg(feature = "track-source")]
            source: None,
        }
    }
    /// Extracts a string slice containing the entire `SharedString`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self
    }
    /// Returns the [`Source`] of this string.
    ///
    /// If the `track-source` feature is not enabled, this function will always return `None`.
    #[must_use]
    pub fn source(&self) -> Option<&Source> {
        #[cfg(feature = "track-source")]
        {
            self.source.as_ref()
        }
        #[cfg(not(feature = "track-source"))]
        {
            None
        }
    }

    /// Sets the path and line number from which this value originated.
    ///
    /// If the `track-source` feature is not enabled, this function is a no-op.
    pub fn set_source(&mut self, #[allow(unused)] source: Source) {
        #[cfg(feature = "track-source")]
        {
            self.source = Some(source)
        }
    }

    /// Efficiently clones the source from `other`.
    ///
    /// If the `track-source` feature is not enabled, this function is a no-op.
    pub fn set_source_from(&mut self, #[allow(unused)] other: &SharedString) {
        #[cfg(feature = "track-source")]
        {
            self.source.clone_from(&other.source);
        }
    }

    /// Clears the path and line number from which this value originated.
    ///
    /// If the `track-source` feature is not enabled, this function is a no-op.
    pub fn clear_source(&mut self) {
        #[cfg(feature = "track-source")]
        {
            self.source = None;
        }
    }

    /// Returns a lowercased version of `self`. Will not allocate if `self` is already lowercase.
    #[must_use]
    pub fn into_lowercase(&self) -> Self {
        // TODO: This requires two iterations over the string, and it's definitely possible
        // to do it in one.
        match into_lowercase(self.value.as_ref()) {
            Cow::Borrowed(_) => self.clone(),
            Cow::Owned(v) => {
                #[allow(unused_mut)]
                let mut retval = SharedString::new(v);
                #[cfg(feature = "track-source")]
                retval.set_source_from(self);
                retval
            }
        }
    }
}

impl std::fmt::Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &*self.value)
    }
}

impl From<&str> for SharedString {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// An immutable shared [`Path`] and line number for tracking the origins of strings and errors.
///
/// This type assumes that line numbers shall not exceed the maximum value of `usize`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Source {
    path: crate::string::Shared<Path>,
    line: usize,
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut swriter = f.debug_struct("Source");
        swriter.field("path", &self.path);
        swriter.field("line", &self.line);
        swriter.finish()
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.path.to_string_lossy(), self.line)
    }
}

impl Source {
    /// Constructs a new [`Source`] from the provided path and line number.
    ///
    /// The line number should be 1-indexed to match convention;
    /// the first line should have a line number of 1 rather than 0.
    #[must_use]
    pub fn new(path: &(impl AsRef<Path> + ?Sized), line: usize) -> Self {
        Source {
            path: crate::string::Shared::from(path.as_ref()),
            line,
        }
    }
    /// Returns a reference to the path and a copy of the line number.
    #[must_use]
    pub fn get(&self) -> (&Path, usize) {
        (crate::string::Shared::as_ref(&self.path), self.line)
    }
    /// As [`Source::get`] but returns a reference to the line number.
    #[must_use]
    pub fn get_ref(&self) -> (&Path, &usize) {
        (crate::string::Shared::as_ref(&self.path), &self.line)
    }
    /// As [`Source::get`] but returns a mut reference to the line number.
    #[must_use]
    pub fn get_mut(&mut self) -> (&Path, &mut usize) {
        (crate::string::Shared::as_ref(&self.path), &mut self.line)
    }
}

impl AsRef<Path> for Source {
    fn as_ref(&self) -> &Path {
        self.get().0
    }
}
