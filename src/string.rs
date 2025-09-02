//! The `SharedString` type and supporting types.

use std::borrow::Cow;

#[cfg(feature = "track-source")]
mod source {
    #[derive(Clone)]
    pub struct Source {
        path: std::sync::Arc<std::path::Path>,
        line: usize,
    }

    impl std::fmt::Debug for Source {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.path.to_string_lossy(), self.line)
        }
    }

    impl Source {
        pub fn new(path: &std::path::Path, line: usize) -> Self {
            Source {
                path: std::sync::Arc::from(path),
                line,
            }
        }
        pub fn get(&self) -> (&std::path::Path, usize) {
            (std::sync::Arc::as_ref(&self.path), self.line)
        }
    }
}

// TODO: Eventually add support for an Arc-like type that uses a thin pointer here.
// Probably not triomphe::ThinArc, since we'd need to use unsafe to use it.

#[derive(Clone, Debug)]
enum SharedStringInner {
    Static(&'static str),
    Owned(std::sync::Arc<str>),
}

impl SharedStringInner {
    #[inline]
    pub fn new(string: &str) -> Self {
        SharedStringInner::Owned(std::sync::Arc::from(string))
    }
    #[inline]
    pub fn get(&self) -> &str {
        match self {
            SharedStringInner::Static(v) => v,
            SharedStringInner::Owned(v) => std::sync::Arc::as_ref(v),
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
    source: Option<source::Source>,
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
        Some(self.value.cmp(&other.value))
    }
}
impl Ord for SharedString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl std::hash::Hash for SharedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.value.as_bytes());
        state.write_u8(0);
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
        &*self.value
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
    pub fn new_static(value: &'static str) -> Self {
        SharedString {
            value: SharedStringInner::Static(value),
            #[cfg(feature = "track-source")]
            source: None,
        }
    }
    #[cfg(feature = "track-source")]
    /// Returns the path to the file and the line number that this value originates from.
    ///
    /// The line number is 1-indexed to match convention;
    /// the first line will have a line number of 1 rather than 0.
    pub fn source(&self) -> Option<(&std::path::Path, usize)> {
        self.source.as_ref().map(source::Source::get)
    }

    #[cfg(feature = "track-source")]
    /// Sets the path and line number from which this value originated.
    ///
    /// The line number should be 1-indexed to match convention;
    /// the first line should have a line number of 1 rather than 0.
    pub fn set_source(&mut self, path: impl AsRef<std::path::Path>, line: usize) {
        self.source = Some(source::Source::new(path.as_ref(), line))
    }

    #[cfg(feature = "track-source")]
    /// Efficiently clones the source from `other`.
    pub fn set_source_from(&mut self, other: &SharedString) {
        self.source = other.source.clone();
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

pub(crate) fn into_lowercase(string: &str) -> std::borrow::Cow<str> {
    // TODO: This requires two iterations over the string, and it's definitely possible
    // to do it in one.
    if string.chars().all(char::is_lowercase) {
        std::borrow::Cow::Borrowed(string)
    } else {
        std::borrow::Cow::Owned(string.to_lowercase())
    }
}
