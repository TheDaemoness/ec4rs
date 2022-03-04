mod alt;
mod globtype;
mod matcher;
mod parser;
mod splitter;

pub use globtype::Glob;
pub use matcher::Matcher;
use splitter::Splitter;

// Really would have preferred to use the glob crate here,
// except EditorConfig has {s1,s2,s3} and {num1..num2}.


pub fn parse(glob: &str) -> Result<Glob, super::ParseError> {
	parser::parse(glob)
}

#[must_use]
pub fn matches(glob: &Glob, path: &std::path::Path) -> bool {
	if let Some(splitter) = Splitter::new(path) {
		matcher::matches(glob, splitter).is_some()
	} else {
		false
	}
}
