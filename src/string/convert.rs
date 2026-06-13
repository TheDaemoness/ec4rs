use crate::string::SharedString;
use std::borrow::Cow;

/// Specialized trait for types whose references can be converted to [`SharedString`]s
/// or possibly [`str`]s.
///
/// This trait exists to enable faster insertions in data structures that use
/// [`SharedString`]s as keys by avoiding a copy if the key already exists.
pub trait ToSharedString {
    /// Converts `self` into a [`SharedString`].
    ///
    /// Callers should assume this will involve full copy of `self`'s content,
    /// though more efficient implementations are often possible.
    fn to_shared_string(self) -> SharedString;
    /// Cheaply get a reference to `self`'s data if possible.
    ///
    /// The returned value, if `Some`, should be equal to the value returned by
    /// [`ToSharedString::to_shared_string`].
    fn try_as_str(&self) -> Option<&str> {
        None
    }
}

impl ToSharedString for &str {
    fn to_shared_string(self) -> SharedString {
        SharedString::new(self)
    }

    fn try_as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ToSharedString for String {
    fn to_shared_string(self) -> SharedString {
        SharedString::new(self.as_str())
    }

    fn try_as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ToSharedString for Cow<'static, str> {
    fn to_shared_string(self) -> SharedString {
        match self {
            Cow::Borrowed(v) => SharedString::new_static(v),
            Cow::Owned(s) => s.to_shared_string(),
        }
    }

    fn try_as_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl ToSharedString for SharedString {
    fn to_shared_string(self) -> SharedString {
        self
    }

    fn try_as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ToSharedString for bool {
    fn to_shared_string(self) -> SharedString {
        SharedString::new_static(if self { "true" } else { "false" })
    }

    fn try_as_str(&self) -> Option<&str> {
        if *self {
            Some("true")
        } else {
            Some("false")
        }
    }
}

impl ToSharedString for usize {
    fn to_shared_string(self) -> SharedString {
        match self {
            0 => SharedString::new_static("0"),
            1 => SharedString::new_static("1"),
            2 => SharedString::new_static("2"),
            4 => SharedString::new_static("4"),
            8 => SharedString::new_static("8"),
            _ => 0.to_string().to_shared_string(),
        }
    }
}

/// A parse error and the [`SharedString`] that failed to parse.
#[derive(Clone, Debug)]
pub struct ParseError<E> {
    /// The error that occurred during parsing.
    pub error: E,
    /// The string on which parsing was attempted.
    pub string: SharedString,
}

impl<E: std::fmt::Display> std::fmt::Display for ParseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", &self.error)
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ParseError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
