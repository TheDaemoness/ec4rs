#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![deny(missing_docs)]

mod ecfile;
mod ecparser;
mod linereader;
mod properties;
pub mod property;
mod readerror;
mod section;
pub mod version;
#[cfg(test)]
mod tests;

pub use ecfile::{EcFile, EcFiles};
pub use ecparser::EcParser;
pub use properties::{Properties, PropertiesSource};
pub use readerror::ReadError;
pub use section::Section;

/// Retrieve the [Properties] for a file at the given path.
///
/// This is the simplest way to use this library in an EditorConfig integration or plugin,
/// and is roughly analogous to the EditorConfig C Core's
/// [editorconfig_parse](https://docs.editorconfig.org/en/master/editorconfig_8h.html#add6bebe96bf90c48fef01cf5300ddf92).
///
/// This function does not canonicalize the path,
/// but will join relative paths onto the current working directory.
pub fn get_config_for(path: impl AsRef<std::path::Path>) -> Result<Properties, ReadError> {
	use std::borrow::Cow;
	let mut path = Cow::from(path.as_ref());
	// TODO: It might be better for this to be the responsibility of EcFiles::open.
	if path.is_relative() {
		path = std::env::current_dir().map_err(ReadError::Io)?.join(&path).into()
	}
	let mut retval = Properties::new();
	EcFiles::open(&path)?.apply_to(&mut retval, &path);
	Ok(retval)
}
