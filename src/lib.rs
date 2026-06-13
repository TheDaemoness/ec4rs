#![doc = include_str!("../rustdoc.md")]
#![forbid(unsafe_code)]
#![deny(clippy::as_conversions)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::wildcard_imports)]
#![deny(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::private_intra_doc_links)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)] // reason = "False positives on EditorConfig".
#![allow(clippy::module_name_repetitions)] // reason = "Affects re-exports from private modules."
#![allow(clippy::must_use_candidate)] // reason = "Too pedantic."
#![allow(clippy::semicolon_if_nothing_returned)] // reason = "Too pedantic."
#![allow(clippy::let_underscore_untyped)] // reason = "Too pedantic."
#![allow(clippy::needless_pass_by_value)] // reason = "FPs on Option<impl AsRef>"
#![allow(clippy::missing_errors_doc)] // reason = "Too verbose in moste cases."
#![cfg_attr(doc_unstable, feature(doc_auto_cfg))]

pub mod cache;
mod error;
mod fallback;
mod file;
pub mod glob;
mod linereader;
mod parser;
mod properties;
pub mod property;
mod section;
pub mod string;
#[cfg(test)]
mod tests;
mod traits;
pub mod version;

pub use error::{Error, ParseError};
pub use file::{ConfigFile, ConfigFiles};
pub use parser::ConfigParser;
pub use properties::{Properties, PropertiesSource};
pub use section::Section;
pub use traits::*;

/// Retrieves the [`Properties`] for a file at the given path.
///
/// This is the simplest way to use this library in an EditorConfig integration or plugin.
///
/// `target_path` should ideally be an absolute path.
/// If it is not, this function will produce an absolute path using [`std::path::absolute`].
/// This may differ from the canonical form of that path.
///
/// EditorConfig files are assumed to be named `.editorconfig`.
/// If not, use [`properties_from_config_of`].
#[inline]
pub fn properties_of<P: crate::glob::Pattern>(
    target_path: impl AsRef<std::path::Path>,
) -> Result<Properties, Error> {
    properties_from_config_of::<P>(target_path.as_ref(), Option::<&std::path::Path>::None)
}

/// Retrieves the [`Properties`] for a file at the given path,
/// expecting EditorConfig files to be named matching `config_name`.
///
/// `target_path` should ideally be an absolute path.
/// If it is not, this function will produce an absolute path using [`std::path::absolute`].
/// This may differ from the canonical form of that path.
///
/// If `config_name` is `None`, uses a default value of `".editorconfig"`.
/// If `config_name` is an absolute path, uses the EditorConfig file at that path.
/// If it's relative, joins it onto every ancestor of `target_path`
/// and looks for config files at those paths.
#[inline]
pub fn properties_from_config_of<P: crate::glob::Pattern>(
    target_path: impl AsRef<std::path::Path>,
    config_name: Option<impl AsRef<std::path::Path>>,
) -> Result<Properties, Error> {
    let mut retval = Properties::new();
    ConfigFiles::<P>::open(
        target_path.as_ref(),
        config_name.as_ref().map(AsRef::as_ref),
    )?
    .apply_to(&mut retval, &target_path)?;
    Ok(retval)
}
