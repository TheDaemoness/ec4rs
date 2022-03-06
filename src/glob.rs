// TODO: All of this glob stuff should be extracted to its own crate.

mod matcher;
mod parser;
mod splitter;
mod stack;

pub use matcher::Matcher;
use splitter::Splitter;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Glob(pub(super) Vec<Matcher>);

impl Glob {
	pub fn new(pattern: &str) -> Glob {
		parser::parse(pattern)
	}

	#[must_use]
	pub fn matches(&self, path: &std::path::Path) -> bool {
		matcher::matches(path, self).is_some()
	}

	pub(super) fn append_char(&mut self, c: char) {
		if c == '/' {
			self.append(Matcher::Sep)
		} else {
			self.append(Matcher::Suffix(c.to_string()))
		}
	}

	pub(super) fn append(&mut self, matcher: Matcher) {
		match &matcher {
			Matcher::Sep => {
				if let Some(Matcher::Sep) = &self.0.last() {
					return
				}
			},
			Matcher::Suffix(suffix) => {
				if let Some(Matcher::Suffix(ref mut prefix)) = self.0.last_mut() {
					prefix.push_str(suffix);
					return
				}
			}
			Matcher::AnySeq(true) => {
				if let Some(Matcher::AnySeq(true)) = &self.0.last() {
					return
				}
			}
			_ => ()
		}
		self.0.push(matcher);
	}

	pub fn append_glob(&mut self, glob: Glob) {
		for matcher in glob.0 {
			self.append(matcher)
		}
	}
}
