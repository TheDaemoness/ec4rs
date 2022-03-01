/// Possible errors that can occur while reading editorconfig files.
#[derive(Debug)]
pub enum ReadError {
	/// End-of-file was reached.
	Eof,
	/// An IO read failure occurred.
	Io(std::io::Error),
	/// An invalid line was read.
	InvalidLine
}

impl std::fmt::Display for ReadError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use ReadError::*;
		match self {
			Eof   => write!(f, "end of data"),
			Io(e) => write!(f, "io failure: {}", e),
			InvalidLine => write!(f, "invalid line")
		}
	}
}

impl std::error::Error for ReadError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		if let ReadError::Io(ioe) = self {
			Some(ioe)
		} else {
			None
		}
	}
}
