use std::path::{Path, PathBuf};

use crate::{
    glob::Pattern, properties::PropertiesSink, ConfigParser, Error, ParseError, PropertiesSource,
    Section,
};

/// Convenience wrapper for an [`ConfigParser`] that reads files.
pub struct ConfigFile<P: Pattern> {
    /// The path to the open file.
    pub path: PathBuf,
    /// A [`ConfigParser`] that reads from the file.
    pub reader: ConfigParser<std::io::BufReader<std::fs::File>, P>,
}

impl<P: Pattern> ConfigFile<P> {
    /// Opens a file for reading and uses it to construct an [`ConfigParser`].
    ///
    /// If the file cannot be opened, wraps the [`std::io::Error`] in a [`ParseError`].
    pub fn open(path: impl AsRef<Path>) -> Result<ConfigFile<P>, ParseError> {
        let file = std::fs::File::open(&path).map_err(ParseError::Io)?;
        let reader = ConfigParser::new_buffered_with_path(file, Some(path.as_ref()))?;
        Ok(ConfigFile {
            path: path.as_ref().to_owned(),
            reader,
        })
    }

    /// Wraps a [`ParseError`] in an [`Error::InFile`].
    ///
    /// Uses the path and current line number from this instance.
    pub fn add_error_context(&self, error: ParseError) -> Error {
        Error::InFile(self.path.clone(), self.reader.line_no(), error)
    }
}

impl<P: Pattern> Iterator for ConfigFile<P> {
    type Item = Result<Section<P>, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next()
    }
}

impl<P: Pattern> std::iter::FusedIterator for ConfigFile<P> {}

impl<P: Pattern> PropertiesSource for &mut ConfigFile<P> {
    /// Adds properties from the file's sections to the specified [`Properties`] map.
    ///
    /// Uses [`ConfigFile::path`] when determining applicability to stop `**` from going too far.
    /// Returns parse errors wrapped in an [`Error::InFile`].
    fn apply_to(
        self,
        props: &mut (impl PropertiesSink + ?Sized),
        path: impl AsRef<Path>,
    ) -> Result<(), crate::Error> {
        let get_parent = || self.path.parent();
        let path = if let Some(parent) = get_parent() {
            let path = path.as_ref();
            path.strip_prefix(parent).unwrap_or(path)
        } else {
            path.as_ref()
        };
        match self.reader.apply_to(props, path) {
            Ok(()) => Ok(()),
            Err(crate::Error::Parse(e)) => Err(self.add_error_context(e)),
            Err(e) => panic!("unexpected error variant {e:?}"),
        }
    }
}

/// Directory traverser for finding and opening EditorConfig files.
///
/// All the contained files are open for reading and have not had any sections read.
/// When iterated over, either by using it as an [`Iterator`]
/// or by calling [`ConfigFiles::iter`],
/// returns [`ConfigFile`]s in the order that they would apply to a [`Properties`] map.
pub struct ConfigFiles<P: Pattern>(Vec<ConfigFile<P>>);

impl<P: Pattern> ConfigFiles<P> {
    /// Searches for EditorConfig files that might apply to a file at the specified path.
    ///
    /// `target_path` should ideally be an absolute path.
    /// If it is not, this function will produce an absolute path using [`std::path::absolute`].
    /// This may differ from the canonical form of that path.
    ///
    /// If `config_name` is `None`, uses a default value of `".editorconfig"`.
    /// If `config_name` is an absolute path, uses the EditorConfig file at that path.
    /// If it's relative, joins it onto every ancestor of `target_path`
    /// and looks for config files at those paths.
    #[allow(clippy::needless_pass_by_value)]
    pub fn open(
        target_path: impl AsRef<Path>,
        config_name: Option<impl AsRef<Path>>,
    ) -> Result<Self, Error> {
        let filename = config_name
            .as_ref()
            .map_or_else(|| ".editorconfig".as_ref(), |f| f.as_ref());
        Ok(ConfigFiles(if filename.is_relative() {
            let path = target_path.as_ref();
            let abs_path = if path.is_absolute() {
                std::borrow::Cow::Borrowed(path)
            } else {
                std::borrow::Cow::Owned(std::path::absolute(path).map_err(Error::InvalidCwd)?)
            };
            let mut path: &Path = &abs_path;
            let mut vec = Vec::new();
            while let Some(dir) = path.parent() {
                if let Ok(file) = ConfigFile::open(dir.join(filename)) {
                    let should_break = file.reader.is_root;
                    vec.push(file);
                    if should_break {
                        break;
                    }
                }
                path = dir;
            }
            vec
        } else {
            // TODO: Better errors.
            vec![ConfigFile::open(filename).map_err(Error::Parse)?]
        }))
    }

    /// Returns an iterator over the contained [`ConfigFiles`].
    pub fn iter(&self) -> impl Iterator<Item = &ConfigFile<P>> {
        self.0.iter().rev()
    }

    // To maintain the invariant that these files have not had any sections read,
    // there is no `iter_mut` method.
}

impl<P: Pattern> Iterator for ConfigFiles<P> {
    type Item = ConfigFile<P>;
    fn next(&mut self) -> Option<ConfigFile<P>> {
        self.0.pop()
    }
}

impl<P: Pattern> std::iter::FusedIterator for ConfigFiles<P> {}

impl<P: Pattern> PropertiesSource for ConfigFiles<P> {
    /// Adds properties from the files' sections to the specified [`Properties`] map.
    ///
    /// Ignores the files' paths when determining applicability.
    fn apply_to(
        self,
        props: &mut (impl PropertiesSink + ?Sized),
        path: impl AsRef<Path>,
    ) -> Result<(), crate::Error> {
        let path = path.as_ref();
        for mut file in self {
            file.apply_to(props, path)?;
        }
        Ok(())
    }
}
