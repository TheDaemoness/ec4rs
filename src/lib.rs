#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![deny(missing_docs)]

mod ecfile;
mod ecparser;
mod fallback;
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
///
/// EditorConfig files are assumed to be named `.editorconfig` unless an override
/// is supplied as the second argument.
pub fn get_config_for(
	path: impl AsRef<std::path::Path>,
	ec_name_override: Option<impl AsRef<std::ffi::OsStr>>
) -> Result<Properties, ReadError> {
	let mut retval = Properties::new();
	match ec_name_override {
		Some(name) => EcFiles::open_with_name(&path, name.as_ref()),
		None       => EcFiles::open(&path)
	}?.apply_to(&mut retval, &path);
	retval.use_fallbacks();
	Ok(retval)
}
