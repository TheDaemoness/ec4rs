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
			'?' => {
				retval = append(retval, Matcher::AnyChar)
			}
			'*' => {
				retval = append(retval, Matcher::AnySeq(matches!(chars.peek(), Some('*'))))
			}
			'[' => {
				(retval, chars) = parse_charclass(retval, chars)?;
			}
			// TODO: {
			_ => {
				retval = append_char(retval, c);
			}
		}
	}
	Ok(retval)
}

fn parse_charclass(
	mut glob: Glob, mut chars: std::iter::Peekable<std::str::Chars<'_>>
) -> Result<(Glob, std::iter::Peekable<std::str::Chars<'_>>), crate::ParseError> {
	let restore = chars.clone();
	let invert = matches!(chars.peek(), Some('!'));
	if invert {
		chars.next();
	}
	let mut found_end: bool = false;
	let mut charclass = std::collections::BTreeSet::<char>::new();
	let mut prev_char: Option<char> = None;
	while let Some(c) = chars.next() {
		match c {
			'\\' => {
				if let Some(c) = chars.next() {
					charclass.insert(c);
					prev_char = Some(c);
				}
			},
			']' => {
				found_end = true;
				break;
			}
			// The spec says nothing about char ranges,
			// but the test suite tests for them.
			// Therefore, EC has them in practice.
			'-' => {
				if let Some(pc) = prev_char {
					// Peek here to handle `-` at the end of a range.
					if let Some(nc_ref) = chars.peek() {
						let mut nc: Option<char> = None;
						match *nc_ref {
							']' => (),
							'\\' => {
								chars.next();
								nc = chars.next().or(Some('\\'));
							}
							other => {
								nc = Some(other);
								chars.next();
							}
						}
						if let Some(nc) = nc {
							for c in pc..=nc {
								charclass.insert(c);
							}
							prev_char = Some(nc);
							continue;
						}
					}
				}
				charclass.insert('-');
				prev_char = Some('-');
			}
			_ => {
				charclass.insert(c);
				prev_char = Some(c);
			}
		}
	}
	if found_end {
		match charclass.len() {
			0 => {
				if invert {
					glob = append(glob, Matcher::AnyChar);
				} else {
					return Err(crate::ParseError::EmptyCharClass);
				}
			}
			1 => {
				glob = append_char(glob, *charclass.iter().next().unwrap());
			}
			_ => {
				glob = append(glob, Matcher::CharClass(charclass, !invert))
			}
		}
	} else {
		chars = restore;
	}
	Ok((glob, chars))
}

fn append_char(mut glob: Glob, c: char) -> Glob {
	if c == '/' {
		append(glob, Matcher::Sep)
	} else if let Matcher::Suffix(suffix) = &mut glob.0 {
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
