//! # ec4rs-glob
//!
//! Refer to the README for an overview of this crate.
//!
//! ## Usage
//!
//! Create a [`Glob`] using [`Glob::new`],
//! then match it against paths with [`Glob::matches`].

mod flatset;
mod matcher;
mod parser;
mod splitter;
mod stack;

#[cfg(test)]
mod tests;

use matcher::Matcher;
use flatset::FlatSet;
use splitter::Splitter;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
/// A single glob pattern.
pub struct Glob(Vec<Matcher>);

impl Default for Glob {
    fn default() -> Self {
        Glob::empty()
    }
}

impl Glob {
    /// Returns an empty `Glob`.
    pub const fn empty() -> Glob {
        Self(Vec::new())
    }

    /// Parses the provided pattern.
    ///
    /// This crate attempts to be maximally permissive in terms of accepted input and will treat
    /// common syntax errors, such as unclosed brackets, as if they were escaped.
    pub fn new(pattern: &str) -> Glob {
        parser::parse(pattern)
    }

    /// Returns `true` if the provided path matches this pattern.
    #[must_use]
    pub fn matches(&self, path: impl AsRef<std::path::Path>) -> bool {
        matcher::matches(path.as_ref(), self).is_some()
    }

    /// Append one [`Matcher`] to `self`.
    fn push(&mut self, matcher: Matcher) {
        // Optimizations, fusing certain kinds of matchers together.
        let push = !match &matcher {
            Matcher::Sep => {
                matches!(&self.0.last(), Some(Matcher::Sep))
            }
            Matcher::Suffix(suffix) => {
                if let Some(Matcher::Suffix(prefix)) = self.0.last_mut() {
                    prefix.push_str(suffix);
                    true
                } else {
                    false
                }
            }
            Matcher::AnySeq(true) => {
                matches!(&self.0.last(), Some(Matcher::AnySeq(false)))
            }
            _ => false,
        };
        if push {
            self.0.push(matcher);
        }
    }

    /// Append one character as-is, with no special functionality..
    fn append_escaped(&mut self, c: char) {
        if c == '/' {
            self.push(Matcher::Sep);
        } else if let Some(Matcher::Suffix(string)) = self.0.last_mut() {
            string.push(c);
        } else {
            // Since we know the Matcher::Suffix case in append() will always be false,
            // we can just save the optimizer the trouble.
            self.0.push(Matcher::Suffix(c.to_string()));
        }
    }

    /// Append all of the matchers from a pattern to `self`.
    fn append_glob(&mut self, glob: Glob) {
        self.0.reserve(glob.0.len());
        for matcher in glob.0 {
            self.push(matcher)
        }
    }
}
