/// Possible errors that can occur while parsing EditorConfig data.
#[derive(Debug)]
pub enum ParseError {
	/// End-of-file was reached.
	Eof,
	/// An IO read failure occurred.
	Io(std::io::Error),
	/// An invalid line was read.
	InvalidLine,
	/// An empty character class was found in a section header.
	EmptyCharClass,
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use ParseError::*;
		match self {
			Eof            => write!(f, "end of data"),
			Io(e)          => write!(f, "io failure: {}", e),
			InvalidLine    => write!(f, "invalid line"),
			EmptyCharClass => write!(f, "empty char class"),
		}
	}
}

impl std::error::Error for ParseError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		if let ParseError::Io(ioe) = self {
			Some(ioe)
		} else {
			None
		}
	}
}

/// All errors that can occur during operation.
#[derive(Debug)]
pub enum Error {
	/// An error occured durign parsing.
	Parse(ParseError),
	/// An error occured during parsing of a file.
	InFile(std::path::PathBuf, usize, ParseError),
	/// The current working directory is invalid (e.g. does not exist).
	InvalidCwd(std::io::Error),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Error::*;
		match self {
			Parse(error)              => write!(f, "{error}"),
			InFile(path, line, error) => write!(f, "{}:{line}: {error}", path.to_string_lossy()),
			InvalidCwd(ioe)           => write!(f, "invalid cwd: {}", ioe),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		use Error::*;
		match self {
			Parse(pe)        => pe.source(),
			InFile(_, _, pe) => pe.source(),
			InvalidCwd(ioe)  => Some(ioe),
		}
	}
}
