mod matcher;
mod parser;
mod splitter;

pub use matcher::Matcher;
use splitter::Splitter;

// Really would have preferred to use the glob crate here,
// except EditorConfig has {s1,s2,s3} and {num1..num2}.

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Glob(pub(self) Matcher, pub(self) Option<Box<Glob>>);

impl Glob {
	pub fn parse(glob: &str) -> Result<Glob, super::Error> {
		parser::parse(glob)
	}

	#[must_use]
	pub fn matches(&self, path: &std::path::Path) -> bool {
		if let Some(splitter) = Splitter::new(path) {
			matcher::matches(self, splitter).is_some()
		} else {
			false
		}
	}
}
