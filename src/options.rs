//! Types for controlling the execution of ec4rs.

/// The glob style to be used in section headers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GlobStyle {
	/// A glob style that passes the editorconfig-core-test suite for the latest version of the specification.
	///
	/// For parsing files named `.editorconfig`, THIS SHOULD BE THE ONLY STYLE USED.
	///
	/// Exhibits a few quirks, some of which are not required by either the spec nor the test suite.
	/// * Incorrect constructions of `[charclass]` or `{a,b}` will instead be parsed literally.
	/// * `/` is not permitted in character classes.
	TestCompliant,
}

impl Default for GlobStyle {
	fn default() -> GlobStyle {
		GlobStyle::TestCompliant
	}
}
