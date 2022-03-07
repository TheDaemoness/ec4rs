use crate::property::Property;

/// A map of property names to property values.
///
/// It features O(log n) lookup and preserves insertion order,
/// as well as convenience methods for type-safe access and parsing of values.
///
/// This structure is case-sensitive.
/// It's the caller's responsibility to ensure all keys and values are lowercased.
#[derive(Clone)]
pub struct Properties {
	keys: Vec<String>,
	map: Vec<(usize, String)>,
}

impl Properties {
	/// Returns an empty [Properties] object.
	pub fn new() -> Properties {
		Properties {
			keys: Vec::new(),
			map: Vec::new(),
		}
	}

	fn get_idxes(&self, key: &str) -> Result<usize, usize> {
		self
			.map
			.as_slice()
			.binary_search_by_key(&key, |(ki, _)| self.keys.get(*ki).unwrap().as_str())
	}

	/// Returns the unparsed "raw" value for the specified key.
	///
	/// Does not test for the "unset" value. Use [RawValue::filter_unset].
	pub fn get_raw_for_key(&self, key: impl AsRef<str>) -> RawValue<'_> {
		let value = self
			.get_idxes(key.as_ref())
			.ok()
			.map(|idx| self.map.get(idx).unwrap().1.as_str())
			.filter(|v| !v.is_empty());
		if let Some(value) = value {
			RawValue::Unknown(value)
		} else {
			RawValue::Unset
		}
	}

	/// Returns the unpared "raw" value for the specified [Property].
	///
	/// Does not test for the "unset" value. Use [RawValue::filter_unset].
	pub fn get_raw<T: Property>(&self) -> RawValue<'_> {
		self.get_raw_for_key(T::key())
	}

	/// Returns the parsed value for the specified [Property].
	///
	/// Does not test for the "unset" value if parsing fails. Use [RawValue::filter_unset].
	pub fn get<T: Property>(&self) -> Result<T, RawValue<'_>> {
		let retval = self.get_raw::<T>();
		retval.parse::<T, false>().or(Err(retval))
	}

	/// Returns an iterator over the key-value pairs, ordered from oldest key to newest key.
	pub fn iter_raw(&self) -> impl Iterator<Item = (&str, &str)> {
		self
			.keys
			.iter()
			.map(|key| (key.as_ref(), self.get_raw_for_key(key).value()))
			.filter_map(|(k, v)| v.map(|v| (k, v)))
	}

	fn get_at(&mut self, idx: usize) -> &mut String {
		&mut self.map.get_mut(idx).unwrap().1
	}

	fn insert_at(&mut self, idx: usize, key: String, value: String) {
		self.map.insert(idx, (self.keys.len(), value));
		self.keys.push(key);
	}

	/// Sets the value for a specified key.
	pub fn insert_raw_for_key(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
		let key_str = key.as_ref();
		match self.get_idxes(key_str) {
			Ok(idx) => {
				*self.get_at(idx) = value.into();
			}
			Err(idx) => {
				self.insert_at(idx, key_str.to_owned(), value.into());
			}
		}
	}

	/// Sets the value for a specified [Property]'s key.
	pub fn insert_raw<T: Property, S: Into<String>>(&mut self, value: S) {
		self.insert_raw_for_key(T::key(), value)
	}

	/// Inserts a specified [Property] into the map.
	///
	/// If the key was already associated with a value, returns the old value.
	pub fn insert<T: Property>(&mut self, prop: T) {
		self.insert_raw_for_key(T::key(), prop.to_string())
	}

	/// Attempts to add a new key-value pair to the map.
	///
	/// If the key was already associated with a value, returns a mutable reference to the old value and does not update the map.
	pub fn try_insert_raw_for_key(&mut self, key: impl AsRef<str>, value: impl Into<String>) -> Result<(), &mut String> {
		let key_str = key.as_ref();
		#[allow(clippy::unit_arg)]
		match self.get_idxes(key_str) {
			Ok(idx) => {
				let valref = self.get_at(idx);
				if valref.is_empty() {
					*valref = value.into();
					Ok(())
				} else {
					Err(valref)
				}
			}
			Err(idx) => Ok(self.insert_at(idx, key_str.to_owned(), value.into())),
		}
	}

	/// Attempts to add a new [Property] to the map with a specified value.
	///
	/// If the key was already associated with a value, returns a mutable reference to the old value and does not update the map.
	pub fn try_insert_raw<T: Property, S: Into<String>>(&mut self, value: S) -> Result<(), &mut String> {
		self.try_insert_raw_for_key(T::key(), value)
	}

	/// Attempts to add a new [Property] to the map.
	///
	/// If the key was already associated with a value, returns a mutable reference to the old value and does not update the map.
	pub fn try_insert<T: Property>(&mut self, prop: T) -> Result<(), &mut String> {
		self.try_insert_raw_for_key(T::key(), prop.to_string())
	}

	/// Add fallback values for certain common key-value pairs.
	///
	/// Used to obtain spec-compliant values for [crate::property::IndentSize]
	/// and [crate::property::TabWidth].
	pub fn use_fallbacks(&mut self) {
		crate::fallback::add_fallbacks(self, false)
	}

	/// Add pre-0.9.0 fallback values for certain common key-value pairs.
	pub fn use_fallbacks_legacy(&mut self) {
		crate::fallback::add_fallbacks(self, true)
	}
}

impl Default for Properties {
	fn default() -> Properties {
		Properties::new()
	}
}

impl<K: AsRef<str>, V: Into<String>> FromIterator<(K, V)> for Properties {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
		let mut result = Properties::new();
		for (k, v) in iter {
			result.insert_raw_for_key(k, v);
		}
		result
	}
}

/// A trait for types that can add properties to a map.
pub trait PropertiesSource {
	/// Adds key-value pairs to a [Properties]
	/// if and only if they apply to a file at the specified path.
	fn apply_to(self, props: &mut Properties, path: impl AsRef<std::path::Path>) -> Result<(), crate::Error>;
}

impl<'a> PropertiesSource for &'a Properties {
	fn apply_to(self, props: &mut Properties, _: impl AsRef<std::path::Path>) -> Result<(), crate::Error> {
		for (k, v) in self.iter_raw() {
			props.insert_raw_for_key(k, v);
		}
		Ok(())
	}
}

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
