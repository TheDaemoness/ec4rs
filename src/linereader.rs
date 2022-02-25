use std::io as io;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Line<'a> {
	Nothing,
	Section(&'a str),
	Pair(&'a str, &'a str),
}

#[derive(Debug)]
pub enum LineFail {
	EOF,
	IoError(io::Error),
	Invalid
}

type LineReadResult<'a> = Result<Line<'a>, LineFail>;

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
				Err(LineFail::Invalid)
			} else {
				Ok(Line::Section(s))
			}
		} else if let Some((key_raw, val_raw)) = l.split_once('=') {
			let key = key_raw.trim_end();
			let val = val_raw.trim_start();
			if key.is_empty() || val.is_empty() {
				Err(LineFail::Invalid)
			} else {
				Ok(Line::Pair(key.trim_end(), val.trim_start()))
			}
		} else {
			Err(LineFail::Invalid)
		}
	}
}

pub struct LineReader<R: io::Read> {
	ticker: usize,
	line: String,
	reader: io::BufReader<R>
}

impl<R: io::Read> LineReader<R> {
	pub fn new(r: R) -> LineReader<R> {
		LineReader {
			ticker: 0,
			line: String::with_capacity(256),
			reader: io::BufReader::new(r)
		}
	}

	pub fn line_no(&self) -> usize {
		self.ticker
	}

	pub fn line(&self) -> &str {
		self.line.as_str()
	}

	// Convenience method for `parse_line(self.line())
	pub fn reparse(&self) -> LineReadResult<'_> {
		parse_line(self.line())
	}

	pub fn next(&mut self) -> LineReadResult<'_> {
		self.line.clear();
		use std::io::BufRead;
		match self.reader.read_line(&mut self.line) {
			Err(e) => Err(LineFail::IoError(e)),
			Ok(0) => Err(LineFail::EOF),
			Ok(_) => {
				self.ticker += 1;
				self.reparse()
			}
		}
	}
}
