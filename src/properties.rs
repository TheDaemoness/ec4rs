use crate::property::Property;

/// A map of property names to property values.
/// When iterated over, returns key/value pairs in the order the keys were first seen.
/// This structure is case-sensitive.
/// It's the caller's responsibility to ensure all keys and values are lowercased.
#[derive(Clone)]
pub struct Properties {
	keys: Vec<String>,
	map: Vec<(usize, String)>
}

impl Properties {
	/// Construct a new, empty Properties object.
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

	/// Retrieve the value for the specified string key.
	pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
		self.get_idxes(key.as_ref()).ok().and_then(|idx| self.map.get(idx).map(|v| v.1.as_ref()))
	}

	/// Return the value for the specified property.
	pub fn property<T: Property>(&self) -> Option<Result<T::Output, &str>> {
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

	/// Set the value for a specified property name.
	/// Returns the old property value.
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

	/// Set the value for a specified property name if it doesn't exist.
	/// If the property already exists, returns a mutable reference to its value, wrap
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
