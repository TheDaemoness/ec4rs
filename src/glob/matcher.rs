use super::{Glob, Splitter};

use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Matcher {
	End,
	Sep,
	AnyChar,
	AnySeq(bool),
	Suffix(String),
	// TODO: Grapheme clusters?
	CharClass(BTreeSet<char>, bool),
	Range(isize, isize),
	Any(Vec<super::Glob>),
}

fn try_match<'a, 'b>(
	mut splitter: Splitter<'a>,
	matcher: &'b Matcher,
	state: &mut super::stack::SaveStack<'a, 'b>,
) -> Option<Splitter<'a>> {
	use Matcher::*;
	Some(match matcher {
		End => splitter.match_end()?,
		Sep => splitter.match_sep()?,
		AnyChar => splitter.match_any(false)?,
		AnySeq(sep) => {
			if let Some(splitter) = splitter.clone().match_any(*sep) {
				state.add_rewind(splitter, matcher);
			}
			splitter
		}
		Suffix(s) => splitter.match_suffix(s.as_str())?,
		CharClass(cs, should_have) => {
			let (splitter, c) = splitter.next_char()?;
			if cs.contains(&c) != *should_have {
				return None;
			}
			splitter
		}
		Range(lower, upper) => {
			let mut q = std::collections::VecDeque::<char>::new();
			let mut allow_zero: bool = true;
			let mut last_ok = splitter.clone();
			while let Some((next_ok, c)) = splitter.next_char() {
				if c.is_numeric() && (c != '0' || allow_zero) {
					last_ok = next_ok.clone();
					allow_zero = c == '0';
					q.push_front(c);
				} else if c == '-' {
					last_ok = next_ok.clone();
					q.push_front('-');
					break;
				} else {
					break;
				}
				splitter = next_ok;
			}
			let i = q.iter().collect::<String>().parse::<isize>().ok()?;
			if i < *lower || i > *upper {
				return None;
			}
			last_ok
		}
		Any(options) => {
			state.add_alts(splitter.clone(), options.as_slice());
			splitter
		}
	})
}

#[must_use]
pub fn matches<'a>(path: &'a std::path::Path, glob: &Glob) -> Option<Splitter<'a>> {
	let mut splitter = super::Splitter::new(path)?;
	let mut state = super::stack::SaveStack::new(&splitter, glob);
	loop {
		if let Some(matcher) = state.globs().next() {
			if let Some(splitter_new) = try_match(splitter, matcher, &mut state) {
				splitter = splitter_new;
			} else if let Some(splitter_new) = state.restore() {
				splitter = splitter_new;
			} else {
				return None;
			}
		} else {
			return Some(splitter);
		}
	}
}
