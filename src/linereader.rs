use std::io as io;

pub enum Line<'a> {
	Nothing,
	Section(&'a str),
	Pair(&'a str, &'a str),
}

pub enum LineFail {
	EOF,
	IoError(io::Error),
	Invalid(usize)
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

	pub fn parse_line(&self) -> Result<Line<'_>, LineFail> {
		let l = self.line.trim();
		if l.is_empty() {
			Ok(Line::Nothing)
		} else if l.starts_with(|c| c == ';' || c == '#') {
			Ok(Line::Nothing)
		} else if let Some(s) = l.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
			Ok(Line::Section(s))
		} else if let Some((key, val)) = l.split_once('=') {
			Ok(Line::Pair(key.trim_end(), val.trim_start()))
		} else {
			Err(LineFail::Invalid(self.ticker))
		}
	}

	pub fn next(&mut self) -> Result<Line<'_>, LineFail> {
		self.line.clear();
		use std::io::BufRead;
		match self.reader.read_line(&mut self.line) {
			Err(e) => Err(LineFail::IoError(e)),
			Ok(0) => Err(LineFail::EOF),
			Ok(_) => {
				self.ticker += 1;
				self.parse_line()
			}
		}
	}
}
