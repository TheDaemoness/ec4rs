mod globtype;
mod matcher;
mod parser;
mod splitter;
mod stack;

pub use globtype::Glob;
pub use matcher::Matcher;
use splitter::Splitter;
use crate::options::GlobStyle;

// Really would have preferred to use the glob crate here,
// except EditorConfig has {s1,s2,s3} and {num1..num2}.

pub fn parse(glob: &str, style: GlobStyle) -> Glob {
	use GlobStyle::*;
	match style {
		TestCompliant => parser::test_compliant(glob)
	}
}

#[must_use]
pub fn matches(glob: &Glob, path: &std::path::Path) -> bool {
	matcher::matches(path, glob).is_some()
}
