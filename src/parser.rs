#[cfg(feature = "ec4rs_glob")]
#[cfg(test)]
mod tests;

use crate::cache::Cache;
use crate::cache::CommonKeyCache;
use crate::cache::CommonValueCache;
use crate::glob::Pattern;
use crate::linereader::LineReader;
use crate::preamble::Preamble;
use crate::properties::PropertiesSink;
use crate::ParseError;
use crate::Section;
use std::io;
use std::path::Path;

/// Parser for the text of an EditorConfig file.
///
/// This struct wraps any [`BufRead`][std::io::BufRead].
/// It eagerly parses the preamble on construction.
/// [`Section`]s may then be parsed by calling [`ConfigParser::read_section`].
pub struct ConfigParser<R: io::BufRead, P: Pattern, K = CommonKeyCache, V = CommonValueCache> {
    /// The preamble for this EditorConfig file.
    pub preamble: Preamble,
    eof: bool,
    reader: LineReader<R>,
    cache_k: K,
    cache_v: V,
    #[allow(clippy::type_complexity)]
    glob_marker: std::marker::PhantomData<fn() -> Result<P, P::Error>>,
    #[cfg(feature = "track-source")]
    path: Option<crate::string::Shared<Path>>,
}

impl<R: io::Read, P: Pattern> ConfigParser<io::BufReader<R>, P, CommonKeyCache, CommonValueCache> {
    /// Convenience function for construction using an unbuffered [`io::Read`].
    ///
    /// See [`ConfigParser::new`].
    pub fn new_buffered(source: R) -> Result<ConfigParser<io::BufReader<R>, P>, ParseError> {
        Self::new(io::BufReader::new(source))
    }
    /// Convenience function for construction using an unbuffered [`io::Read`]
    /// which is assumed to be a file at `path`.
    ///
    /// See [`ConfigParser::new_with_path`].
    pub fn new_buffered_with_path(
        source: R,
        path: Option<&(impl AsRef<Path> + ?Sized)>,
    ) -> Result<ConfigParser<io::BufReader<R>, P>, ParseError> {
        Self::new_with_path(io::BufReader::new(source), path.as_ref())
    }
}

impl<R: io::BufRead, P: Pattern> ConfigParser<R, P, CommonKeyCache, CommonValueCache> {
    /// Constructs a new [`ConfigParser`] and reads the preamble from the provided source,
    /// which is assumed to be a file at `path`.
    ///
    /// Returns `Ok` if the preamble was parsed successfully,
    /// otherwise returns `Err` with the error that occurred during reading.
    pub fn new_with_path(
        buf_source: R,
        #[allow(unused)] path: Option<&(impl AsRef<Path> + ?Sized)>,
    ) -> Result<Self, ParseError> {
        let mut reader = LineReader::new(buf_source);
        let mut is_root = false;
        let eof = loop {
            use crate::linereader::Line;
            match reader.next_line() {
                Err(ParseError::Eof) => break true,
                Err(e) => return Err(e),
                Ok(Line::Nothing) => (),
                Ok(Line::Section(_)) => break false,
                Ok(Line::Pair(k, v)) => {
                    if "root".eq_ignore_ascii_case(k) {
                        if let Ok(b) = v.to_ascii_lowercase().parse::<bool>() {
                            is_root = b;
                        }
                    }
                    // Quietly ignore unknown properties.
                }
            }
        };
        Ok(ConfigParser {
            preamble: Preamble::new().with_root(is_root),
            eof,
            reader,
            cache_k: CommonKeyCache,
            cache_v: CommonValueCache,
            glob_marker: std::marker::PhantomData,
            #[cfg(feature = "track-source")]
            path: path.map(|p| crate::string::Shared::from(p.as_ref())),
        })
    }
    /// Constructs a new [`ConfigParser`] and reads the preamble from the provided source.
    ///
    /// Returns `Ok` if the preamble was parsed successfully,
    /// otherwise returns `Err` with the error that occurred during reading.
    pub fn new(buf_source: R) -> Result<Self, ParseError> {
        Self::new_with_path(buf_source, Option::<&Path>::None)
    }
}

impl<R: io::BufRead, P: Pattern, K, V> ConfigParser<R, P, K, V> {
    /// Returns `true` if there may be another section to read.
    pub fn has_more(&self) -> bool {
        !self.eof
    }

    /// Returns the current line number.
    pub fn line_no(&self) -> usize {
        self.reader.line_no()
    }

    /// Returns a version of `self` with the provided cache for keys.
    pub fn with_key_cache<C>(self, cache_k: C) -> ConfigParser<R, P, C, V> {
        ConfigParser {
            preamble: self.preamble,
            eof: self.eof,
            reader: self.reader,
            cache_k,
            cache_v: self.cache_v,
            glob_marker: std::marker::PhantomData,
            #[cfg(feature = "track-source")]
            path: self.path,
        }
    }

    /// Returns a version of `self` with the provided cache for values.
    pub fn with_value_cache<C>(self, cache_v: C) -> ConfigParser<R, P, K, C> {
        ConfigParser {
            preamble: self.preamble,
            eof: self.eof,
            reader: self.reader,
            cache_k: self.cache_k,
            cache_v,
            glob_marker: std::marker::PhantomData,
            #[cfg(feature = "track-source")]
            path: self.path,
        }
    }
}

impl<R: io::BufRead, P: Pattern, K: Cache, V: Cache> ConfigParser<R, P, K, V> {
    /// Parses a [`Section`], reading more if needed.
    pub fn read_section(&mut self) -> Result<Section<P>, ParseError> {
        use crate::linereader::Line;
        if self.eof {
            return Err(ParseError::Eof);
        }
        if let Ok(Line::Section(header)) = self.reader.reparse() {
            let mut section = Section::new(header);
            loop {
                // Get line_no here to avoid borrowing issues, increment for 1-based indices.
                #[cfg(feature = "track-source")]
                let line_no = self.reader.line_no() + 1;
                match self.reader.next_line() {
                    Err(e) => {
                        self.eof = true;
                        break if matches!(e, ParseError::Eof) {
                            Ok(section)
                        } else {
                            Err(e)
                        };
                    }
                    Ok(Line::Section(_)) => break Ok(section),
                    Ok(Line::Nothing) => (),
                    Ok(Line::Pair(k, v)) => {
                        #[allow(unused_mut)]
                        let mut v = self.cache_v.get_shared_string(v);
                        #[cfg(feature = "track-source")]
                        if let Some(path) = self.path.as_ref() {
                            v.set_source(path.clone(), line_no);
                        }
                        section.insert(self.cache_k.get_shared_string(k), v);
                    }
                }
            }
        } else {
            Err(ParseError::InvalidLine)
        }
    }
}

impl<R: io::BufRead, P: Pattern, K: Cache, V: Cache> Iterator for ConfigParser<R, P, K, V> {
    type Item = Result<Section<P>, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.read_section() {
            Ok(r) => Some(Ok(r)),
            Err(ParseError::Eof) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<R: io::BufRead, P: Pattern, K: Cache, V: Cache> std::iter::FusedIterator
    for ConfigParser<R, P, K, V>
{
}

impl<R: io::BufRead, P: Pattern, K: Cache, V: Cache> crate::PropertiesSource
    for &mut ConfigParser<R, P, K, V>
{
    fn apply_to(
        self,
        props: &mut (impl PropertiesSink + ?Sized),
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error> {
        let path = path.as_ref();
        // TODO: Don't buffer entire sections.
        for section_result in self {
            match section_result {
                Ok(section) => {
                    let _ = section.apply_to(props, path);
                }
                Err(error) => return Err(crate::Error::Parse(error)),
            }
        }
        Ok(())
    }
}
