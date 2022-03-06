#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![deny(missing_docs)]

mod ecfile;
mod ecparser;
mod error;
mod fallback;
mod glob;
mod linereader;
mod properties;
pub mod property;
mod section;
pub mod version;
#[cfg(test)]
mod tests;

pub use ecfile::{EcFile, EcFiles};
pub use ecparser::EcParser;
pub use error::{Error, ParseError};
pub use properties::{Properties, PropertiesSource, RawValue};
pub use section::Section;

/// Retrieve the [Properties] for a file at the given path.
///
/// This is the simplest way to use this library in an EditorConfig integration or plugin.
///
/// This function does not canonicalize the path,
/// but will join relative paths onto the current working directory.
///
/// EditorConfig files are assumed to be named `.editorconfig`
/// unless an override is supplied as the second argument.
pub fn get_config_for(
	path: impl AsRef<std::path::Path>,
	filename_override: Option<impl AsRef<std::ffi::OsStr>>
) -> Result<Properties, Error> {
	let mut retval = Properties::new();
	EcFiles::open(&path, filename_override)?.apply_to(&mut retval, &path)?;
	retval.use_fallbacks();
	Ok(retval)
}
