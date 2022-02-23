use patricia_tree::PatriciaMap;

use crate::property::Property;

/// A map of property names to property values.
/// When iterated over, returns key/value pairs in the order the keys were first seen.
/// This structure is case-sensitive.
/// It's the caller's responsibility to ensure all keys and values are lowercased.
#[derive(Clone)]
pub struct Properties {
	keys: Vec<String>,
	map: PatriciaMap<String>
}

impl Properties {
	/// Construct a new, empty Properties object.
	pub fn new() -> Properties {
		Properties {
			keys: Vec::new(),
			map: PatriciaMap::new()
		}
	}

	/// Retrieve the value for the specified string key.
	pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
		self.map.get(key.as_ref()).map(|x| x.as_str())
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

	/// Set the value for a specified string key.
	pub fn set(&mut self, key: impl AsRef<str>, value: String) -> &mut Self {
		let key_str = key.as_ref();
		if self.map.insert(key_str, value).is_none() {
			self.keys.push(key_str.to_owned())
		}
		self
	}
}

impl<K: AsRef<str>, V: Into<String>> FromIterator<(K, V)> for Properties {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
		let mut result = Properties::new();
		for (k, v) in iter {
			result.set(k, v.into());
		}
		result
	}
}
