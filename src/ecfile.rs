use std::path::{Path, PathBuf};

use crate::{EcParser, Properties, PropertiesSource, ReadError, Section};

/// Convenience wrapper for an [EcParser] that reads files.
pub struct EcFile {
	/// The path to the open file.
	pub path: PathBuf,
	/// An [EcParser] that reads from the file.
	pub reader: EcParser<std::io::BufReader<std::fs::File>>
}

impl EcFile {
	/// Opens a file for reading and uses it to construct an [EcParser].
	///
	/// If the file cannot be opened, wraps the [std::io::Error] in a [ReadError].
	pub fn open(path: impl Into<PathBuf>) -> Result<EcFile, ReadError> {
		let path = path.into();
		let file = std::fs::File::open(&path).map_err(ReadError::Io)?;
		let reader = EcParser::new_buffered(file)?;
		Ok(EcFile {
			path,
			reader
		})
	}
}

impl Iterator for EcFile {
	type Item = Result<Section, ReadError>;
	fn next(&mut self) -> Option<Self::Item> {
		self.reader.next()
	}
}

impl std::iter::FusedIterator for EcFile {}

impl PropertiesSource for &mut EcFile {
	/// Adds properties from the file's sections to the specified [Properties] map.
	///
	/// Ignores [EcFile::path] when determining applicability.
	fn apply_to(self, props: &mut Properties, path: impl AsRef<std::path::Path>) {
		self.reader.apply_to(props, path)
	}
}

/// A directory traverser for finding and opening EditorConfig files.
///
/// All the contained files are open for reading and have not had any sections read.
/// When iterated over, either by using it as an [Iterator]
/// or by calling [EcFiles::iter],
/// returns [EcFile]s in the order that they would apply to a [Properties] map.
pub struct EcFiles(Vec<EcFile>);

impl EcFiles {
	/// Searches for EditorConfig files that might apply to a file at the specified path.
	///
	/// This associated function requires you to specify what EditorConfig
	/// files are named. To use the default of `.editorconfig`, use [EcFiles::open] instead.
	pub fn open_with_name(
		path: impl AsRef<Path>,
		ec_filename: &std::ffi::OsStr
	) -> Result<EcFiles, ReadError> {
		use std::borrow::Cow;
		let mut abs_path = Cow::from(path.as_ref());
		if abs_path.is_relative() {
			abs_path = std::env::current_dir().map_err(ReadError::Io)?.join(&path).into()
		}
		let mut path = abs_path.as_ref();
		let mut vec = Vec::new();
		while let Some(dir) = path.parent() {
			if let Ok(reader) = EcFile::open(dir.join(ec_filename)) {
				vec.push(reader)
			}
			// TODO: EcFile errors are suppressed here.
			// Maybe store them in a field or something.
			path = dir;
		}
		Ok(EcFiles(vec))
	}

	/// Searches for EditorConfig files named `.editorconfig`
	/// that might apply to a file at the specified path.
	pub fn open(path: impl AsRef<Path>) -> Result<EcFiles, ReadError> {
		Self::open_with_name(path, ".editorconfig".as_ref())
	}

	/// Returns an iterator over the contained [EcFiles].
	pub fn iter(&self) -> impl Iterator<Item = &EcFile> {
		self.0.iter().rev()
	}

	// To maintain the invariant that these files have not had any sections read,
	// there is no `iter_mut` method.
}

impl Iterator for EcFiles {
	type Item = EcFile;
	fn next(&mut self) -> Option<EcFile> {
		self.0.pop()
	}
}

impl std::iter::FusedIterator for EcFiles {}

impl PropertiesSource for EcFiles {
		/// Adds properties from the files' sections to the specified [Properties] map.
	///
	/// Ignores the files' paths when determining applicability.
	fn apply_to(self, props: &mut Properties, path: impl AsRef<std::path::Path>) {
		let path = path.as_ref();
		for EcFile { mut reader , ..} in self {
			reader.apply_to(props, path)
		}
	}
}
