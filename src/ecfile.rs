use std::path::{Path, PathBuf};

use crate::{EcParser, Properties, PropertiesSource, ParseError, Section, Error};

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
	/// If the file cannot be opened, wraps the [std::io::Error] in a [ParseError].
	pub fn open(path: impl Into<PathBuf>) -> Result<EcFile, ParseError> {
		let path = path.into();
		let file = std::fs::File::open(&path).map_err(ParseError::Io)?;
		let reader = EcParser::new_buffered(file)?;
		Ok(EcFile {
			path,
			reader
		})
	}

	/// Wrap a [ParseError] in an [Error::InFile].
	pub fn add_error_context(&self, error: ParseError) -> Error {
		Error::InFile(
			self.path.clone(),
			self.reader.line_no(),
			error
		)
	}
}

impl Iterator for EcFile {
	type Item = Result<Section, ParseError>;
	fn next(&mut self) -> Option<Self::Item> {
		self.reader.next()
	}
}

impl std::iter::FusedIterator for EcFile {}

impl PropertiesSource for &mut EcFile {
	/// Adds properties from the file's sections to the specified [Properties] map.
	///
	/// Uses [EcFile::path] when determining applicability to stop `**` from going too far.
	/// Returns parse errors wrapped in an [Error::InFile].
	fn apply_to(
		self,
		props: &mut Properties,
		path: impl AsRef<Path>
	) -> Result<(), crate::Error> {
		let get_parent = || self.path.parent();
		let path = if let Some(parent) = get_parent() {
			let path = path.as_ref();
			path.strip_prefix(parent).unwrap_or(path)
		} else {
			path.as_ref()
		};
		match self.reader.apply_to(props, path) {
			Ok(()) => Ok(()),
			Err(crate::Error::Parse(e)) => Err(self.add_error_context(e)),
			Err(e) => panic!("unexpected error variant {:?}", e)
		}
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
	/// This function does not canonicalize the path,
	/// but will join relative paths onto the current working directory.
	///
	/// EditorConfig files are assumed to be named `.editorconfig`
	/// unless an override is supplied as the second argument.
	pub fn open(
		path: impl AsRef<Path>,
		filename_override: Option<impl AsRef<std::ffi::OsStr>>
	) -> Result<EcFiles, Error> {
		use std::borrow::Cow;
		let filename = if let Some(ref fno) = filename_override {
			let oss = fno.as_ref();
			let path: &Path = oss.as_ref();
			path.file_name()
		} else {
			None
		}.unwrap_or_else(|| ".editorconfig".as_ref());
		let mut abs_path = Cow::from(path.as_ref());
		if abs_path.is_relative() {
			abs_path = std::env::current_dir().map_err(Error::InvalidCwd)?.join(&path).into()
		}
		let mut path = abs_path.as_ref();
		let mut vec = Vec::new();
		while let Some(dir) = path.parent() {
			if let Ok(file) = EcFile::open(dir.join(filename)) {
				let should_break = file.reader.is_root;
				vec.push(file);
				if should_break {
					break;
				}
			}
			// TODO: EcFile errors are suppressed here.
			// Maybe store them in a field or something.
			path = dir;
		}
		Ok(EcFiles(vec))
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
	fn apply_to(
		self,
		props: &mut Properties,
		path: impl AsRef<Path>
	) -> Result<(), crate::Error> {
		let path = path.as_ref();
		for mut file in self {
			file.apply_to(props, path)?;
		}
		Ok(())
	}
}
