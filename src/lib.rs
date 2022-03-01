#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![deny(missing_docs)]

mod ecfile;
mod ecreader;
mod linereader;
mod properties;
pub mod property;
mod readerror;
mod section;
#[cfg(test)]
mod tests;

/// The semantic version of the EditorConfig spec this library complies with.
pub const EC_VERSION: &str = "0.14.0";

pub use ecfile::{EcFile, EcFiles};
pub use ecreader::EcReader;
pub use properties::Properties;
pub use readerror::ReadError;
pub use section::Section;
