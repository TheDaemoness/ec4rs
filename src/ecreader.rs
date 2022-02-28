use std::io;

use crate::ReadError;
use crate::Section;
use crate::linereader::LineReader;

pub struct EcReader<R: io::BufRead> {
	pub is_root: bool,
	pub eof: bool,
	reader: LineReader<R>
}

impl<R: io::Read> EcReader<io::BufReader<R>> {
	pub fn new_buffered(r: R) -> Result<EcReader<io::BufReader<R>>, ReadError> {
		Self::new(io::BufReader::new(r))
	}
}

impl<R: io::BufRead> EcReader<R> {
	pub fn new(r: R) -> Result<EcReader<R>, ReadError> {
		let mut reader = LineReader::new(r);
		let (is_root, eof) = reader.read_prelude()?;
		Ok(EcReader {is_root, reader, eof})
	}
	pub fn read_section(&mut self) -> Result<Section, ReadError> {
		let (section, eof) = self.reader.read_section()?;
		self.eof = eof;
		Ok(section)
	}
}

impl<R: io::BufRead> Iterator for EcReader<R> {
	type Item = crate::Section;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.eof {
			self.read_section().ok()
		} else {None}
	}
}
