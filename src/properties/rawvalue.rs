
/// Wrapper around unparsed values in [Properties].
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RawValue<'a> {
	/// Absence of a value.
	Unset,
	/// The value "unset", which has special behavior for all common properties.
	UnsetExplicit,
	/// An unparsed value.
	Unknown(&'a str),
}

impl<'a> RawValue<'a> {
	/// Returns `UnsetExplicit` if self matches `Unknown("unset")`.
	/// Otherwise, returns `self`.
	///
	/// Comparison is done case-insensitively.
	#[must_use]
	pub fn filter_unset(self) -> Self {
		use RawValue::*;
		match self {
			Unknown(v) => {
				if "unset".eq_ignore_ascii_case(v) {
					UnsetExplicit
				} else {
					self
				}
			}
			v => v,
		}
	}
	/// Returns true if the value is unset, including by a value of "unset".
	#[must_use]
	pub const fn is_unset(&self) -> bool {
		use RawValue::*;
		matches!(self, Unset | UnsetExplicit)
	}

	/// Converts this `RawValue` into a [Result].
	///
	/// The `bool` in the `Err` variant is true if
	/// the key-value pair was unset by a value of "unset".
	pub const fn into_result(&self) -> Result<&'a str, bool> {
		use RawValue::*;
		match self {
			Unknown(s)    => Ok(s),
			UnsetExplicit => Err(true),
			Unset         => Err(false),
		}
	}

	/// Returns the unparsed value as a `&str`.
	///
	/// If the key-value pair was unset explicitly,
	/// returns `Some("unset")`.
	pub const fn value(&self) -> Option<&'a str> {
		use RawValue::*;
		match self {
			Unset         => None,
			UnsetExplicit => Some("unset"),
			Unknown(s)    => Some(s),
		}
	}
	/// Attempts to parse the contained value.
	///
	/// For convenience, this function may lowercase a contained value before parsing.
	/// Specify whether it should be lowercased as the second generic argument.
	///
	/// If the value is unset, returns `Err(None)`.
	pub fn parse<T: std::str::FromStr, const LOWERCASE: bool>(&self) -> Result<T, Option<T::Err>> {
		use RawValue::*;
		match self {
			Unknown(v) => if LOWERCASE {
				T::from_str(v.to_lowercase().as_str())
			} else {
				T::from_str(v)
			}
			.map_err(Some),
			_ => Err(None),
		}
	}
}
