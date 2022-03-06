use crate::Properties;

use std::path::Path;

/// A section of an EditorConfig file.
pub struct Section {
	pattern: crate::glob::Glob,
	props: crate::Properties,
}

impl Section {
	/// Constrcts a new Section that applies to files matching the specified pattern.
	pub fn new(pattern: &str, style: crate::options::GlobStyle) -> Section {
		Section {
			pattern: crate::glob::parse(pattern, style),
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
			val.as_ref()
		);
	}
	/// Returns true if and only if this section applies to a file at the specified path.
	#[must_use]
	pub fn applies_to(&self, path: impl AsRef<Path>) -> bool {
		crate::glob::matches(&self.pattern, path.as_ref())
	}
}

impl crate::PropertiesSource for &Section {
	/// Adds this section's properties to a [Properties].
	///
	/// This implementation is infallible.
	fn apply_to(
		self,
		props: &mut Properties,
		path: impl AsRef<std::path::Path>
	) -> Result<(), crate::Error> {
		let path_ref = path.as_ref();
		if self.applies_to(path_ref) {
			let _ = self.props.apply_to(props, path_ref);
		}
		Ok(())
	}
}
