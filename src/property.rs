//! Enums for common EditorConfig properties.
//!
//! This crate contains every current universal property specified by standard,
//! plus others that are common enough to be worth supporting.
//! All of them are non-exhaustive enums in order to support future additions to the standard
//! as well as handle the common special value `"unset"`.

mod language_tag;

pub use language_tag::*;

use super::{PropertyKey, PropertyValue};
use crate::string::SharedString;

use std::fmt::Display;

/// Error for common property parse failures.
#[derive(Clone, Copy, Debug)]
pub struct UnknownValueError;

impl Display for UnknownValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown value")
    }
}

impl std::error::Error for UnknownValueError {}

// TODO: Deduplicate these macros a bit?

macro_rules! property_choice {
    ($prop_id:ident, $name:literal; $(($variant:ident, $string:literal)),+) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
        #[doc = concat!("The [`",$name,"`](https://github.com/editorconfig/editorconfig/wiki/EditorConfig-Properties#",$name,") property.")]
        #[allow(missing_docs)]
        #[non_exhaustive]
        pub enum $prop_id {
            #[default]
            Unset,
            $($variant),+
        }

        impl std::str::FromStr for $prop_id {
            type Err = UnknownValueError;
            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                match &*crate::string::into_lowercase(raw) {
                    $($string => Ok($prop_id::$variant),)+
                    _ => Err(UnknownValueError)
                }
            }
        }

        impl crate::string::ToSharedString for $prop_id {
            fn to_shared_string(self) -> SharedString {
                SharedString::new_static(match self {
                    $prop_id::Unset => "unset",
                    $($prop_id::$variant => $string),*
                })
            }

            fn try_as_str(&self) -> Option<&str> {
                Some(match self {
                    $prop_id::Unset => "unset",
                    $($prop_id::$variant => $string),*
                })
            }
        }

        impl PropertyValue for $prop_id {}

        impl PropertyKey for $prop_id {
            fn key() -> &'static str {$name}
        }

        impl Display for $prop_id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $prop_id::Unset => "unset".fmt(f),
                    $($prop_id::$variant => $string.fmt(f)),*
                }
            }
        }
    }
}

macro_rules! property_valued {
    (
        $prop_id:ident, $name:literal, $value_type:ty;
        $(($variant:ident, $string:literal)),*
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
        #[doc = concat!("The [`",$name,"`](https://github.com/editorconfig/editorconfig/wiki/EditorConfig-Properties#",$name,") property.")]
        #[allow(missing_docs)]
        #[non_exhaustive]
        pub enum $prop_id {
            #[default]
            Unset,
            Value($value_type)
            $(,$variant)*
        }

        impl std::str::FromStr for $prop_id {
            type Err = UnknownValueError;
            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                match &*crate::string::into_lowercase(raw) {
                    "unset" => Ok($prop_id::Unset),
                    $($string => Ok($prop_id::$variant),)*
                    v => v.parse::<$value_type>().map(Self::Value).or(Err(UnknownValueError))
                }
            }
        }

        impl crate::string::ToSharedString for $prop_id {
            fn to_shared_string(self) -> SharedString {
                match self {
                    $prop_id::Unset => SharedString::new_static("unset"),
                    $prop_id::Value(v) => SharedString::new(v.to_string()),
                    $($prop_id::$variant => SharedString::new_static($string)),*
                }
            }

            fn try_as_str(&self) -> Option<&str> {
                match self {
                    $prop_id::Unset => Some("unset"),
                    $prop_id::Value(_) => None,
                    $($prop_id::$variant => Some($string)),*
                }
            }
        }

        impl PropertyValue for $prop_id {}

        impl PropertyKey for $prop_id {
            fn key() -> &'static str {$name}
        }

        impl Display for $prop_id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $prop_id::Unset => "unset".fmt(f),
                    $prop_id::Value(v) => v.fmt(f),
                    $($prop_id::$variant => $string.fmt(f)),*
                }
            }
        }
    }
}

property_choice! {
    IndentStyle, "indent_style";
    (Tabs, "tab"),
    (Spaces, "space")
}

// NOTE:
// The spec and the wiki disagree on the valid range of indent/tab sizes.
// The spec says "whole numbers" for both,
// whereas the wiki says "an integer"/"a positive integer" respectively.
// This implementation follows the spec strictly here.
// Notably, it will happily consider sizes of 0 valid.

property_valued! {IndentSize, "indent_size", usize; (UseTabWidth, "tab")}
property_valued! {TabWidth, "tab_width", usize;}

property_choice! {
    EndOfLine, "end_of_line";
    (Lf,   "lf"),
    (CrLf, "crlf"),
    (Cr,   "cr")
}

property_choice! {
    Charset, "charset";
    (Utf8,    "utf-8"),
    (Latin1,  "latin1"),
    (Utf16Le, "utf-16le"),
    (Utf16Be, "utf-16be"),
    (Utf8Bom, "utf-8-bom")
}

property_valued! {TrimTrailingWs, "trim_trailing_whitespace", bool;}
property_valued! {FinalNewline, "insert_final_newline", bool;}
property_valued! {MaxLineLen, "max_line_length", usize;}

/// The `spelling_language` property added by EditorConfig 0.16.
///
/// This type's [`PropertyValue`] implementation, by default,
/// adheres strictly to the EditorConfig spec.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum SpellingLanguage {
    #[default]
    Unset,
    Value(LanguageTag),
}

impl std::str::FromStr for SpellingLanguage {
    type Err = UnknownValueError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.eq_ignore_ascii_case("unset") {
            Ok(SpellingLanguage::Unset)
        } else {
            LanguageTag::try_from(SharedString::new(raw)).map(SpellingLanguage::Value)
        }
    }
}

impl crate::string::ToSharedString for SpellingLanguage {
    fn to_shared_string(self) -> SharedString {
        match self {
            SpellingLanguage::Unset => crate::string::UNSET.clone(),
            SpellingLanguage::Value(retval) => SharedString::new(retval.to_string()),
        }
    }

    fn try_as_str(&self) -> Option<&str> {
        match self {
            SpellingLanguage::Unset => Some("unset"),
            SpellingLanguage::Value(_) => None,
        }
    }
}

impl PropertyValue for SpellingLanguage {
    fn from_shared_string(raw: &SharedString) -> Result<Self, Self::Err> {
        if raw.eq_ignore_ascii_case("unset") {
            Ok(SpellingLanguage::Unset)
        } else {
            LanguageTag::try_from(raw.clone()).map(SpellingLanguage::Value)
        }
    }
}

impl PropertyKey for SpellingLanguage {
    fn key() -> &'static str {
        "spelling_language"
    }
}

impl Display for SpellingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpellingLanguage::Unset => crate::string::UNSET.fmt(f),
            SpellingLanguage::Value(v) => v.fmt(f),
        }
    }
}

/// All the keys of the standard properties.
///
/// Can be used to determine if a property is defined in the specification or not.
pub static STANDARD_KEYS: &[&str] = &[
    "indent_size",
    "indent_style",
    "tab_width",
    "end_of_line",
    "charset",
    "trim_trailing_whitespace",
    "insert_final_newline",
    "spelling_language",
    // NOT "max_line_length".
];
