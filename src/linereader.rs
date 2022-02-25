use std::io as io;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Line<'a> {
	/// Either a comment or an empty line.
	Nothing,
	/// A section header, e.g. `[something.rs]`
	Section(&'a str),
	/// A propery/key-value pair, e.g. `indent_size = 2`
	Pair(&'a str, &'a str),
}

#[derive(Debug)]
pub enum LineReadFail {
	Eof,
	IoError(io::Error),
	Invalid
}

type LineReadResult<'a> = Result<Line<'a>, LineReadFail>;

/// Identifies the line type and extracts relevant slices.
/// Does not do any lowercasing or anything beyond basic validation.
///
/// It's usually not necessary to call this function directly.
pub fn parse_line(line: &str) -> LineReadResult<'_> {
	let mut l = line.trim_start();
	if l.starts_with(|c| c == ';' || c == '#') {
		Ok(Line::Nothing)
	} else {
		l = l.trim_end();
		if l.is_empty() {
			Ok(Line::Nothing)
		} else if let Some(s) = l.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
			if s.is_empty() {
				Err(LineReadFail::Invalid)
			} else {
				Ok(Line::Section(s))
			}
		} else if let Some((key_raw, val_raw)) = l.split_once('=') {
			let key = key_raw.trim_end();
			let val = val_raw.trim_start();
			if key.is_empty() || val.is_empty() {
				Err(LineReadFail::Invalid)
			} else {
				Ok(Line::Pair(key.trim_end(), val.trim_start()))
			}
		} else {
			Err(LineReadFail::Invalid)
		}
	}
}

/// A struct for extracting valid INI-like lines from text,
/// suitable for initial parsing of individual .editorconfig files.
/// Does minimal validation and does not modify the input text in any way.
pub struct LineReader<R: io::Read> {
	ticker: usize,
	line: String,
	reader: io::BufReader<R>
}

impl<R: io::Read> LineReader<R> {
	/// Constructs a new line reader.
	pub fn new(r: R) -> LineReader<R> {
		LineReader {
			ticker: 0,
			line: String::with_capacity(256),
			reader: io::BufReader::new(r)
		}
	}

	/// Returns the line number of the contained line.
	pub fn line_no(&self) -> usize {
		self.ticker
	}

	/// Returns a reference to the contained line.
	pub fn line(&self) -> &str {
		self.line.as_str()
	}

	/// Parses the contained line using [parse_line].
	///
	/// It's usually not necessary to call this method.
	/// See [LineReader::next].
	pub fn reparse(&self) -> LineReadResult<'_> {
		parse_line(self.line())
	}

	/// Read and parse the next line from the stream.
	pub fn next(&mut self) -> LineReadResult<'_> {
		self.line.clear();
		use std::io::BufRead;
		match self.reader.read_line(&mut self.line) {
			Err(e) => Err(LineReadFail::IoError(e)),
			Ok(0) => Err(LineReadFail::Eof),
			Ok(_) => {
				self.ticker += 1;
				self.reparse()
			}
		}
	}
}
