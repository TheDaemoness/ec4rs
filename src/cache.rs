//! Caching of parsed values.

use crate::string::SharedString;

/// Trait for types that represent a collection of `SharedString`s.
pub trait Cache {
    /// Convert `value` into a `SharedString`.
    ///
    /// This method intentionally takes `&self` in order to better-support
    /// static immutable caches. If a mutable cache is needed,
    /// put one in a [`std::cell::RefCell`] or [`std::sync::Mutex`].
    fn get_shared_string(&self, value: &str) -> SharedString;
}

impl<C: Cache + ?Sized, T: std::ops::Deref<Target = C>> Cache for T {
    fn get_shared_string(&self, value: &str) -> SharedString {
        self.deref().get_shared_string(value)
    }
}

/// A cache containing a set of property keys often seen in EditorConfig.
pub struct CommonKeyCache;

/// A cache containing a set of property values often seen in EditorConfig.
pub struct CommonValueCache;

macro_rules! make_cache_match {
    ($value:ident { $($lit:literal,)+ }) => {
        match $value {
            $(
                $lit => SharedString::new_static($lit),
            )+
            _ => SharedString::new($value)
        }
    }
}

impl Cache for CommonKeyCache {
    fn get_shared_string(&self, value: &str) -> SharedString {
        if let Some(value) = crate::property::lookup_standard_key(value) {
            SharedString::new_static(value)
        } else {
            make_cache_match! {value {
                "max_line_length",
            }}
        }
    }
}

impl Cache for CommonValueCache {
    fn get_shared_string(&self, value: &str) -> SharedString {
        make_cache_match!(value {
            "",
            "0",
            "1",
            "2",
            "4",
            "8",
            "cr",
            "crlf",
            "false",
            "latin1",
            "lf",
            "space",
            "tab",
            "true",
            "unset",
            "utf-16be",
            "utf-16le",
            "utf-8",
            "utf-8-bom",
        })
    }
}
