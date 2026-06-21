use crate::string::Source;

/// Possible errors that can occur while parsing EditorConfig data.
#[derive(Debug)]
pub enum ParseError {
    /// End-of-file was reached.
    Eof,
    /// An IO read failure occurred.
    Io(std::io::Error),
    /// An invalid line was read.
    InvalidLine,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Eof => write!(f, "end of data"),
            ParseError::Io(e) => write!(f, "io failure: {e}"),
            ParseError::InvalidLine => write!(f, "invalid line"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::Io(e) => Some(e),
            _ => None,
        }
    }
}

/// All errors that can occur during operation.
#[derive(Debug)]
pub enum Error {
    /// An error occured during parsing.
    Parse(ParseError, Option<Source>),
    /// The current working directory is invalid (e.g. does not exist).
    InvalidCwd(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(error, None) => write!(f, "{error}"),
            Error::Parse(error, Some(source)) => {
                let (path, line) = source.get();
                write!(f, "{}:{}: {}", path.to_string_lossy(), line, error)
            }
            Error::InvalidCwd(ioe) => write!(f, "invalid cwd: {ioe}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(pe, _) => pe.source(),
            Error::InvalidCwd(ioe) => Some(ioe),
        }
    }
}
