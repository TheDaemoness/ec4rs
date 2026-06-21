#[cfg(test)]
mod tests;

use crate::ParseError;

use std::io;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Line<'a> {
    /// Either a comment or an empty line.
    Nothing,
    /// A section header, e.g. `[something.rs]`
    Section(&'a str),
    /// A propery/key-value pair, e.g. `indent_size = 2`
    Pair(&'a str, &'a str),
}

type LineReadResult<'a> = Result<Line<'a>, ParseError>;

/// Identifies the line type and extracts relevant slices.
/// Does not do any lowercasing or anything beyond basic validation.
///
/// Using [`LineReader`] may be more convenient than using this function directly.
///
/// If a line begins with `U+FEFF` (ZWNBSP, more commonly used as the BOM),
/// this function strips it.
pub fn parse_line(line: &str) -> LineReadResult<'_> {
    let line = line.strip_prefix("\u{feff}").unwrap_or(line).trim_start();
    if line.is_empty() || line.starts_with(is_comment) {
        Ok(Line::Nothing)
    } else if line.starts_with('[') {
        let Some(bracket) = line.rfind(']') else {
            return Err(ParseError::InvalidLine);
        };
        // Tolerate inline comments after section headers.
        if bracket + 1 < line.len() {
            for c in line[bracket + 1..].chars() {
                if is_comment(c) {
                    break;
                } else if !c.is_whitespace() && !c.is_control() {
                    return Err(ParseError::InvalidLine);
                }
            }
        }
        let s = &line[1..bracket];
        if s.is_empty() {
            Err(ParseError::InvalidLine)
        } else {
            Ok(Line::Section(s))
        }
    } else if let Some((key_raw, val_raw)) = line.split_once('=') {
        let key = key_raw.trim_end();
        if key.is_empty() {
            Err(ParseError::InvalidLine)
        } else {
            Ok(Line::Pair(key, val_raw.trim()))
        }
    } else {
        Err(ParseError::InvalidLine)
    }
}

/// Struct for extracting valid INI-like lines from text.
///
/// Suitable for initial parsing of individual .editorconfig files.
/// Does minimal validation and does not modify the input text in any way.
pub struct LineReader<R> {
    ticker: usize,
    line: String,
    reader: R,
}

impl<R> LineReader<R> {
    /// Constructs a new line reader.
    pub fn new(r: R) -> LineReader<R> {
        LineReader {
            ticker: 0,
            line: String::with_capacity(256),
            reader: r,
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

    /// Parses the contained line using [`parse_line`].
    ///
    /// It's usually not necessary to call this method.
    /// See [`LineReader::next_line`].
    pub fn reparse(&self) -> LineReadResult<'_> {
        parse_line(self.line())
    }
}

impl<R: io::BufRead> LineReader<R> {
    /// Reads and parses the next line from the stream.
    pub fn next_line(&mut self) -> LineReadResult<'_> {
        self.line.clear();
        match self.reader.read_line(&mut self.line) {
            Err(e) => Err(ParseError::Io(e)),
            Ok(0) => Err(ParseError::Eof),
            Ok(_) => {
                self.ticker += 1;
                self.reparse()
            }
        }
    }
}

fn is_comment(c: char) -> bool {
    c == ';' || c == '#'
}
