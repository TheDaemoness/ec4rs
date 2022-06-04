mod iter;
mod rawvalue;

pub use iter::{Iter, IterMut};
pub use rawvalue::RawValue;

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
	pairs: Vec<(String, String)>,
	/// A list of indices of `pairs`, sorted by the key of the pair each index refers to.
	idxes: Vec<usize>,
}

impl Properties {
	/// Returns an empty [Properties] object.
	pub fn new() -> Properties {
		Properties {
			pairs: Vec::new(),
			idxes: Vec::new(),
		}
	}

	/// Returns either the index of the pair with the desired key in `pairs`,
	/// or the index to insert a new index into `index`.
	fn find_idx(&self, key: &str) -> Result<usize, usize> {
		self
			.idxes
			.as_slice()
			.binary_search_by_key(&key, |ki| self.pairs[*ki].0.as_str())
			.map(|idx| self.idxes[idx])
	}

	/// Returns the unparsed "raw" value for the specified key.
	///
	/// Does not test for the "unset" value. Use [RawValue::filter_unset].
	pub fn get_raw_for_key(&self, key: impl AsRef<str>) -> RawValue<'_> {
		let value = self
			.find_idx(key.as_ref())
			.ok()
			.map(|idx| self.pairs[idx].1.as_str())
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

	/// Returns an iterator over the key-value pairs.
	///
	/// Pairs are returned from oldest to newest.
	pub fn iter(&self) -> Iter<'_> {
		Iter(self.pairs.iter())
	}

	/// Returns an iterator over the key-value pairs that allows mutation of the values.
	///
	/// Pairs are returned from oldest to newest.
	pub fn iter_mut(&mut self) -> IterMut<'_> {
		IterMut(self.pairs.iter_mut())
	}

	fn get_at_mut(&mut self, idx: usize) -> &mut String {
		&mut self.pairs.get_mut(idx).unwrap().1
	}

	fn insert_at(&mut self, idx: usize, key: String, value: String) {
		self.idxes.insert(idx, self.pairs.len());
		self.pairs.push((key, value));
	}

	/// Sets the value for a specified key.
	pub fn insert_raw_for_key(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
		let key_str = key.as_ref();
		match self.find_idx(key_str) {
			Ok(idx) => {
				*self.get_at_mut(idx) = value.into();
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
		match self.find_idx(key_str) {
			Ok(idx) => {
				let valref = self.get_at_mut(idx);
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
		for (k, v) in self.iter() {
			props.insert_raw_for_key(k, v.value().unwrap_or_default());
		}
		Ok(())
	}
}
