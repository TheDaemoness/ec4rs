use super::Chars;
use crate::glob::{Glob, Matcher};

pub fn parse(mut glob: Glob, mut chars: Chars<'_>) -> (Glob, Chars<'_>) {
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
			}
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
		// Remove slashes for the sake of consistent behavior.
		charclass.remove(&'/');
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
			_ => glob.append(Matcher::CharClass(charclass, !invert)),
		}
	} else {
		chars = restore;
		glob.append_char('[');
	}
	(glob, chars)
}
