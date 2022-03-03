use super::{Glob, Matcher};

pub fn parse(glob: &str) -> Result<Glob, crate::ParseError> {
	let mut retval = Glob(Matcher::Sep, None);
	let mut chars = glob.chars().peekable();
	while let Some(c) = chars.next() {
		match c {
			'\\' => {
				if let Some(escaped) = chars.next() {
					retval = append_char(retval, escaped);
				}
			}
			'/' => {
				retval = append(retval, Matcher::Sep)
			}
			'?' => {
				retval = append(retval, Matcher::AnyChar)
			}
			'*' => {
				retval = append(retval, Matcher::AnySeq(matches!(chars.peek(), Some('*'))))
			}
			//TODO: [ and {
			_ => {
				retval = append_char(retval, c);
			}
		}
	}
	Ok(retval)
}

fn append_char(mut glob: Glob, c: char) -> Glob {
	if let Matcher::Suffix(suffix) = &mut glob.0 {
		suffix.push(c);
		glob
	} else {
		append(glob, Matcher::Suffix(c.to_string()))
	}
}

fn append(glob: Glob, matcher: Matcher) -> Glob {
	match &matcher {
		Matcher::Sep => {
			if let Matcher::Sep = &glob.0 {
				return glob
			}
		},
		Matcher::AnySeq(true) => {
			if let Matcher::AnySeq(true) = &glob.0 {
				return glob
			}
		}
		_ => ()
	}
	Glob(matcher, Some(Box::new(glob)))
}
