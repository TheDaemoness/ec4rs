use crate::section::Section;

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

#[derive(Clone, Copy)]
pub enum MaybeLast<V> {
	Last(V),
	More(V)
}

#[derive(Debug)]
pub enum LineReadError {
	Eof,
	IoError(io::Error),
	Invalid
}

type LineReadResult<'a> = Result<Line<'a>, LineReadError>;

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
				Err(LineReadError::Invalid)
			} else {
				Ok(Line::Section(s))
			}
		} else if let Some((key_raw, val_raw)) = l.split_once('=') {
			let key = key_raw.trim_end();
			let val = val_raw.trim_start();
			if key.is_empty() || val.is_empty() {
				Err(LineReadError::Invalid)
			} else {
				Ok(Line::Pair(key.trim_end(), val.trim_start()))
			}
		} else {
			Err(LineReadError::Invalid)
		}
	}
}

/// A struct for extracting valid INI-like lines from text,
/// suitable for initial parsing of individual .editorconfig files.
/// Does minimal validation and does not modify the input text in any way.
pub struct LineReader<R: io::BufRead> {
	ticker: usize,
	line: String,
	reader: R
}

impl<R: io::BufRead> LineReader<R> {
	/// Constructs a new line reader.
	pub fn new(r: R) -> LineReader<R> {
		LineReader {
			ticker: 0,
			line: String::with_capacity(256),
			reader: r
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

	/// Reads and parse the next line from the stream.
	pub fn next_line(&mut self) -> LineReadResult<'_> {
		self.line.clear();
		use std::io::BufRead;
		match self.reader.read_line(&mut self.line) {
			Err(e) => Err(LineReadError::IoError(e)),
			Ok(0) => Err(LineReadError::Eof),
			Ok(_) => {
				self.ticker += 1;
				self.reparse()
			}
		}
	}

	/// Reads the prelude.
	///
	/// Reads lines until the next section header or EOF is found.
	/// Every [Line::Pair] line is considered invalid unless
	/// the key is `root` and the value parses into a [`bool`].
	///
	/// Returns a pair of booleans in the Ok variant.
	/// The first is true if and only if a `root = true` line was found.
	/// The second is true if and only if EOF was NOT was reached while reading.
	pub fn read_prelude(&mut self) -> Result<(bool, bool), LineReadError> {
		let mut is_root = false;
		loop {
			match self.next_line() {
				Err(LineReadError::Eof) => return Ok((is_root, false)),
				Err(e)                  => return Err(e),
				Ok(Line::Nothing)       => (),
				Ok(Line::Section(_))    => return Ok((is_root, true)),
				Ok(Line::Pair(k, v))    => {
					if "root".eq_ignore_ascii_case(k) {
						if let Ok(b) = v.parse::<bool>() {
							is_root = b;
							continue
						}
					}
					return Err(LineReadError::Invalid)
				}
			}
		}
	}

	/// Reads a section.
	///
	/// Expects the current line to be a section header.
	/// Reads lines until the next section header or EOF is found.
	///
	/// The boolean that's returned with the [Section] is true
	/// if and only if EOF was NOT reached while reading.
	pub fn read_section(&mut self) -> Result<(Section, bool), LineReadError> {
		if let Ok(Line::Section(header)) = self.reparse() {
			let mut section = Section::new(header);
			loop {
				match self.next_line() {
					Err(LineReadError::Eof) => return Ok((section, false)),
					Err(e)                  => return Err(e),
					Ok(Line::Section(_))    => return Ok((section, true)),
					Ok(Line::Nothing)       => (),
					Ok(Line::Pair(k,v))     => {
						section.insert(k,v);
					}
				}
			}
		} else {
			Err(LineReadError::Invalid)
		}
	}
}
