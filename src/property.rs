pub trait Property {
	type Output;
	fn key() -> &'static str;
	fn parse_value(raw: &str) -> Option<Self::Output>;
}

macro_rules! property_basic_custom {
	($prop_id:ident, $name:literal, $parse_as:ty, $parse_arg:ident, $parse_block:block) => {
		pub struct $prop_id;
		impl Property for $prop_id {
			type Output = $parse_as;
			fn key() -> &'static str {$name}
			fn parse_value($parse_arg: &str) -> Option<Self::Output> {
				$parse_block
			}
		}
	}
}

macro_rules! property_basic {
	($prop_id:ident, $name:literal, $parse_as:ty) => {
		property_basic_custom!{$prop_id, $name, $parse_as, raw, {
			raw.parse::<$parse_as>().ok()
		}
	}}
}

macro_rules! property_basic_option {
	($prop_id:ident, $name:literal, $parse_as:ty, $disable:literal) => {
		property_basic_custom!{$prop_id, $name, Option<$parse_as>, raw, {
			if raw == $disable {
				Some(None)
			} else {
				raw.parse::<$parse_as>().ok().map(Some)
			}
		}
	}}
}

macro_rules! property_enum {
	($prop_id:ident, $name:literal, $(($variant:ident, $string:literal)),+) => {
		#[derive(Clone, Copy, PartialEq, Eq, Debug)]
		#[repr(u8)]
		pub enum $prop_id {$($variant),+}
		impl Property for $prop_id {
			type Output = $prop_id;
			fn key() -> &'static str {$name}
			fn parse_value(raw: &str) -> Option<$prop_id> {
				match raw {
					$($string => Some($prop_id::$variant)),+,
					_ => None
				}
			}
		}
	}
}

property_enum!{
	IndentStyle, "indent_style",
	(Tabs, "tab"),
	(Spaces, "space")
}

//NOTE:
//The spec and the wiki disagree on the valid range of indent/tab sizes.
//The spec says "whole numbers" for both,
//whereas the wiki says "an integer"/"a positive integer" respectively.
//This implementation follows the spec strictly here.
//Notably, it will happily consider sizes of 0 valid.

property_basic_option!{IndentSize, "indent_size", usize, "tab"}

property_basic!{TabWidth, "tab_width", usize}

property_enum!{
	EndOfLine, "end_of_line",
	(Lf,   "lf"),
	(CrLf, "crlf"),
	(Cr,   "cr")
}

property_enum!{
	Charset, "charset",
	(Utf8,    "utf-8"),
	(Latin1,  "latin1"),
	(Utf16Le, "utf-16le"),
	(Utf16Be, "utf-16be"),
	(Utf8Bom, "utf-8-bom")
}

property_basic!{TrimTrailingWs, "trim_trailing_whitespace", bool}
property_basic!{InsertFinalNewline, "insert_final_newline", bool}
property_basic_option!{MaxLineLen, "max_line_length", usize, "off"}
