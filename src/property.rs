//! Type-safe key-value pair parsers.
//!
//! Includes the [crate::property::Property] trait
//! as well as instances for common properties.

/// A trait for types that represent properties.
///
/// Used for enums or newtypes that are associated with string keys,
/// that also know how to parse themselves from string values.
pub trait Property: Sized {
	/// The string key for this property.
	///
	/// Used to look up the value in a [crate::Properties] map.
	fn key() -> &'static str;
	/// Parses a string value into itself.
	fn parse_value(raw: &str) -> Option<Self>;
}

//TODO: Deduplicate these macros a bit?

macro_rules! property_choice {
	($prop_id:ident, $name:literal; $(($variant:ident, $string:literal)),+) => {
		#[derive(Clone, Copy, PartialEq, Eq, Debug)]
		#[repr(u8)]
		#[doc = concat!("The [`",$name,"`](https://github.com/editorconfig/editorconfig/wiki/EditorConfig-Properties#",$name,") property.")]
		#[allow(missing_docs)]
		pub enum $prop_id {$($variant),+}
		impl Property for $prop_id {
			fn key() -> &'static str {$name}
			fn parse_value(raw: &str) -> Option<Self> {
				match raw {
					$($string => Some($prop_id::$variant),)+
					_ => None
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
		#[derive(Clone, Copy, PartialEq, Eq, Debug)]
		#[doc = concat!("The [`",$name,"`](https://github.com/editorconfig/editorconfig/wiki/EditorConfig-Properties#",$name,") property.")]
		#[allow(missing_docs)]
		pub enum $prop_id {
			Value($value_type)
			$(,$variant)*
		}
		impl Property for $prop_id {
			fn key() -> &'static str {$name}
			fn parse_value(raw: &str) -> Option<Self> {
				match raw {
					$($string => Some($prop_id::$variant),)*
					_ => raw.parse::<$value_type>().ok().map(Self::Value)
				}
			}
		}
	}
}

property_choice!{
	IndentStyle, "indent_style";
	(Tabs, "tab"),
	(Spaces, "space")
}

//NOTE:
//The spec and the wiki disagree on the valid range of indent/tab sizes.
//The spec says "whole numbers" for both,
//whereas the wiki says "an integer"/"a positive integer" respectively.
//This implementation follows the spec strictly here.
//Notably, it will happily consider sizes of 0 valid.

property_valued!{IndentSize, "indent_size", usize; (UseTabWidth, "tab")}
property_valued!{TabWidth, "tab_width", usize;}

property_choice!{
	EndOfLine, "end_of_line";
	(Lf,   "lf"),
	(CrLf, "crlf"),
	(Cr,   "cr")
}

property_choice!{
	Charset, "charset";
	(Utf8,    "utf-8"),
	(Latin1,  "latin1"),
	(Utf16Le, "utf-16le"),
	(Utf16Be, "utf-16be"),
	(Utf8Bom, "utf-8-bom")
}

property_valued!{TrimTrailingWs, "trim_trailing_whitespace", bool;}
property_valued!{FinalNewline, "insert_final_newline", bool;}
property_valued!{MaxLineLen, "max_line_length", usize; (Off, "off")}
