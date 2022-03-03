use super::Glob;

use std::collections::BTreeSet;

// TODO: Non-recursive implementation of this algorithm.

// The spec requires an implementation to support section headers up to 1024 chars.
// The smallest repeatable sequence of chars that will result in recursion is 2:
// `*` followed by a non-special character.
// Therefore a recursion depth of 512 should be enough for the worst-case scenario.
const MAX_RECURSE_DEPTH: usize = 512;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Matcher {
	Sep,
	AnyChar,
	AnySeq(bool),
	Suffix(String),
	CharSet(BTreeSet<char>, bool),
	Range(isize, isize),
	Any(Vec<super::Glob>)
}

fn try_match<'a, 'b>(
	glob: &'a Glob,
	splitter: super::Splitter<'b>,
	stack: &mut Vec<RestorePoint<'a, 'b>>
) -> Option<super::Splitter<'b>> {
	use Matcher::*;
	let Glob(matcher, _) = glob;
	Some(match matcher {
		Sep => splitter.match_sep()?,
		AnyChar => splitter.match_any(false)?,
		AnySeq(sep) => {
			let s = splitter.match_any(*sep)?;
			stack.push(RestorePoint{glob, splitter: s.clone(), /*idx: 0*/});
			s
		},
		Suffix(s) => splitter.match_suffix(s.as_str())?,
		_ => return None //TODO: Other patterns.
	})
}

pub fn matches<'a, 'b>(
	mut glob: &'a Glob,
	mut splitter: super::Splitter<'b>
) -> Option<super::Splitter<'b>> {
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

pub struct RestorePoint<'a, 'b> {
	glob: &'a Glob,
	splitter: super::Splitter<'b>,
	/*idx: usize*/
}
