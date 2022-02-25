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
	map: Vec<(usize, String)>
}

impl Properties {
	/// Returns an empty [Properties] object.
	pub fn new() -> Properties {
		Properties {
			keys: Vec::new(),
			map: Vec::new()
		}
	}

	fn get_idxes(&self, key: &str) -> Result<usize, usize> {
		self.map.as_slice().binary_search_by_key(&key, |(ki, _)| {
				self.keys.get(*ki).unwrap().as_str()
		})
	}

	/// Returns the string value for the specified key.
	pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
		self.get_idxes(key.as_ref()).ok().and_then(|idx| self.map.get(idx).map(|v| v.1.as_ref()))
	}

	/// Returns the parsed value for the specified [Property].
	/// Returns `None` if there is no matching key-value pair in this map.
	/// Returns `Some(Err)` if the key exists but has an unknown/invalid value.
	pub fn property<T: Property>(&self) -> Option<Result<T::Value, &str>> {
		self.get(T::key()).map(|v| T::parse_value(v).ok_or(v))
	}

	/// Returns an iterator over the key-value pairs, ordered from oldest key to newest key.
	pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
		self.keys.iter().map(|key| {
			(key.as_ref(), self.get(key).unwrap())
		})
	}

	fn get_at(&mut self, idx: usize) -> &mut String {
		&mut self.map.get_mut(idx).unwrap().1
	}

	fn insert_at(&mut self, idx: usize, key: String, value: String) {
		self.map.insert(idx, (self.keys.len(), value));
		self.keys.push(key);
	}

	/// Sets the value for a specified key.
	/// Returns the old value if present.
	pub fn insert(&mut self, key: impl AsRef<str>, value: impl Into<String>) -> Option<String> {
		let key_str = key.as_ref();
		match self.get_idxes(key_str) {
			Ok(idx) => {
				let mut retval = value.into();
				std::mem::swap(self.get_at(idx), &mut retval);
				Some(retval)
			}
			Err(idx) => {
				self.insert_at(idx, key_str.to_owned(), value.into());
				None
			}
		}
	}

	/// Attempts to add a new key-value pair.
	/// Returns `Ok(())` if the key was not already associated with a value.
	/// Returns a mutable reference to the old value otherwise, and does not update the map.
	pub fn try_insert(&mut self, key: impl AsRef<str>, value: impl Into<String>) -> Result<(), &mut String> {
		let key_str = key.as_ref();
		#[allow(clippy::unit_arg)]
		match self.get_idxes(key_str) {
			Ok(idx)  => Err(self.get_at(idx)),
			Err(idx) => Ok(self.insert_at(idx, key_str.to_owned(), value.into()))
		}
	}
}

impl Default for Properties {
	fn default() -> Properties {Properties::new()}
}

impl<K: AsRef<str>, V: Into<String>> FromIterator<(K, V)> for Properties {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
		let mut result = Properties::new();
		for (k, v) in iter {
			result.insert(k, v);
		}
		result
	}
}
