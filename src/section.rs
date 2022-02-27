use crate::Properties;

use std::path::{Path, PathBuf};

/// A section of an EditorConfig file.
pub struct Section {
	pattern: PathBuf, //TODO: Replace with a glob matcher.
	props: crate::Properties,
}

impl Section {
	/// Constrcts a new Section that applies to files matching the specified pattern.
	pub fn new(pattern: &str) -> Section {
		Section {
			pattern: pattern.into(),
			props: crate::Properties::new()
		}
	}
	/// Returns an immutable reference to the internal [Properties] map.
	pub fn props(&self) -> &Properties {
		&self.props
	}
	/// Adds a key-value pair to this section.
	///
	/// Lowercases both the key and the value.
	pub fn insert(&mut self, key: impl AsRef<str>, val: impl AsRef<str>) {
		self.props.insert(
			key.as_ref().to_lowercase(),
			val.as_ref().to_lowercase()
		);
	}
	/// Returns true if and only if this section applies to a file at the specified path.
	pub fn applies_to(&self, path: impl AsRef<Path>) -> bool {
		//TODO: Replace with glob matching.
		path.as_ref().ends_with(&self.pattern)
	}
	/// Appends this section's properties to a [Properties]
	/// if and only if this section applies to a file at the specified path.
	pub fn apply_to(&self, path: impl AsRef<Path>, props: &mut Properties) {
		if self.applies_to(path) {
			for (k, v) in self.props.iter() {
				props.insert(k, v);
			}
		}
	}
}
