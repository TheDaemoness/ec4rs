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
