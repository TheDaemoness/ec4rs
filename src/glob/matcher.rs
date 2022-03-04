use super::{Glob, Splitter};

use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Matcher {
	Sep,
	AnyChar,
	AnySeq(bool),
	Suffix(String),
	// TODO: Grapheme clusters?
	CharClass(BTreeSet<char>, bool),
	Range(isize, isize),
	Any(Vec<super::Glob>)
}

fn try_match<'a, 'b>(
	glob: &'a Glob,
	splitter: Splitter<'b>,
	stack: &mut Vec<RestorePoint<'a, 'b>>
) -> Option<Splitter<'b>> {
	use Matcher::*;
	let Glob(matcher, _) = glob;
	Some(match matcher {
		Sep => splitter.match_sep()?,
		AnyChar => splitter.match_any(false)?,
		AnySeq(sep) => {
			if let Some(splitter) = splitter.clone().match_any(*sep) {
				stack.push(RestorePoint{glob, splitter, /*idx: 0*/});
			}
			splitter
		},
		Suffix(s) => splitter.match_suffix(s.as_str())?,
		CharClass(cs, should_have) => {
			let (splitter, c) = splitter.next_char()?;
			if cs.contains(&c) != *should_have {
				return None;
			}
			splitter
		}
		_ => return None //TODO: Other patterns.
	})
}

pub fn matches<'a, 'b>(
	mut glob: &'a Glob,
	mut splitter: Splitter<'b>
) -> Option<Splitter<'b>> {
	let mut stack = Vec::<RestorePoint<'a, 'b>>::new();
	/*let mut idx = 0usize;*/
	loop {
		if let Some(splitter_new) = try_match(glob, splitter, &mut stack) {
			splitter = splitter_new;
			if let Some(ref next) = glob.1 {
				glob = next
			} else {
				break Some(splitter)
			}
		} else if let Some(restore) = stack.pop() {
			RestorePoint{glob, splitter/*, idx*/} = restore;
		} else {
			break None;
		}
	}
}

struct RestorePoint<'a, 'b> {
	glob: &'a Glob,
	splitter: Splitter<'b>,
	/*idx: usize*/
}
