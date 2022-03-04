mod test_compliant;

pub use test_compliant::parse as test_compliant;

use super::{Glob, Matcher};

type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn parse_range(mut chars: Chars<'_>) -> Option<(isize, isize, Chars<'_>)> {
	let parse_int = |chars: &mut Chars<'_>, breaker: char| {
		let mut num: String = chars.next().filter(|c| c.is_numeric() || *c == '-')?.to_string();
		loop {
			let c = chars.next()?;
			if c.is_numeric() {
				num.push(c)
			} else if c == breaker {
				break Some(num);
			} else {
				return None;
			}
		}
	};
	let num_a = parse_int(&mut chars, '.')?;
	if !matches!(chars.next(), Some('.')) {
		return None;
	}
	let num_b: String = parse_int(&mut chars, '}')?;
	Some((num_a.parse().ok()?, num_b.parse().ok()?, chars))
}

fn parse_charclass(
	mut glob: Glob, mut chars: std::iter::Peekable<std::str::Chars<'_>>
) -> (Glob, std::iter::Peekable<std::str::Chars<'_>>) {
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
							// TODO: Cleanup.
							if pc == '/' || nc == '/' {
								chars = restore;
								glob.append_char('/');
								return (glob, chars);
							}
							for c in pc..=nc {
								if c != '/' {
									charclass.insert(c);
								}
							}
							prev_char = Some(nc);
							continue;
						}
					}
				}
				charclass.insert('-');
				prev_char = Some('-');
			},
			_ => {
				charclass.insert(c);
				prev_char = Some(c);
			}
		}
	}
	if found_end {
		if charclass.contains(&'/') {
			chars = restore;
			glob.append_char('[');
		} else {
			match charclass.len() {
				0 => {
					if invert {
						glob.append(Matcher::AnyChar);
					} else {
						glob.append_char('[');
						glob.append_char(']');
					}
				}
				1 => glob.append_char(*charclass.iter().next().unwrap()),
				_ => glob.append(Matcher::CharClass(charclass, !invert))
			}
		}
	} else {
		chars = restore;
		glob.append_char('[');
	}
	(glob, chars)
}
