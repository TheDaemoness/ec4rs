use crate::glob::{Glob, Matcher};

pub fn parse(glob: &str) -> Glob {
	let mut retval = Glob(vec![]);
	let mut stack = AltStack::new();
	let mut found_sep: bool = false;
	for segment in glob.split('/') {
		retval.append_char('/');
		found_sep = retval.0.len() > 1;
		let mut chars = segment.chars().peekable();
		while let Some(c) = chars.next() {
			match c {
				'\\' => {
					if let Some(escaped) = chars.next() {
						retval.append_char(escaped);
					}
				}
				'?' => retval.append(Matcher::AnyChar),
				'*' => retval.append(Matcher::AnySeq(matches!(chars.peek(), Some('*')))),
				'[' => {
					(retval, chars) = super::parse_charclass(retval, chars, true);
				}
				'{' => {
					if let Some((a, b, chars_new)) = super::parse_range(chars.clone()) {
						chars = chars_new;
						retval.append(Matcher::Range(
							// Reading the spec strictly,
							// a compliant implementation must handle cases where
							// the left integer is greater than the right integer.
							std::cmp::min(a, b),
							std::cmp::max(a, b)
						));
					} else {
						stack.push(retval);
						retval = Glob(vec![]);
					}
				}
				',' => {
					if let Some(rejected) = stack.add_alt(retval) {
						retval = rejected;
						retval.append_char(',');
					} else {
						retval = Glob(vec![]);
					}
				}
				'}' => {
					let add_brace: bool;
					(retval, add_brace) = stack.add_alt_and_pop(retval);
					if add_brace {
						retval.append_char('}');
					}
				}
				_ => retval.append_char(c)
			}
		}
	}
	loop {
		let is_empty: bool;
		(retval, is_empty) = stack.join_and_pop(retval);
		if is_empty {
			break;
		}
	}
	if found_sep {
		*retval.0.first_mut().unwrap() = Matcher::End;
	}
	retval
}

pub struct AltStack(Vec<AltBuilder>);

impl AltStack {
	pub fn new() -> AltStack {
		AltStack(vec![])
	}
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn push(&mut self, glob: Glob) {
		self.0.push(AltBuilder::new(glob));
	}

	/// Adds a glob to the top builder of the stack.
	///
	/// Returns the glob if there is no builder on the stack.
	#[must_use]
	pub fn add_alt(&mut self, glob: Glob) -> Option<Glob> {
		if let Some(ab) = self.0.last_mut() {
			ab.add(glob);
			None
		} else {
			Some(glob)
		}
	}

	pub fn join_and_pop(&mut self, glob: Glob) -> (Glob, bool) {
		match self.0.pop() {
			Some(mut builder) => {
				builder.add(glob);
				(builder.join(), self.is_empty())
			}
			None => (glob, true)
		}
	}

	pub fn add_alt_and_pop(&mut self, glob: Glob) -> (Glob, bool) {
		match self.0.pop() {
			Some(mut builder) => {
				builder.add(glob);
				(builder.build(), false)
			}
			None => (glob, true)
		}
	}
}

pub struct AltBuilder {
	glob: Glob,
	options: Vec<Glob>
}

impl AltBuilder {
	pub fn new(glob: Glob) -> AltBuilder {
		AltBuilder {
			glob,
			options: vec![]
		}
	}
	pub fn add(&mut self, glob: Glob) {
		self.options.push(glob);
	}
	pub fn build(mut self) -> Glob {
		match self.options.len() {
			0 => {
				self.glob.append_char('{');
				self.glob.append_char('}');
				self.glob
			}
			1 => {
				self.glob.append_char('{');
				for matcher in self.options.pop().unwrap().0 {
					self.glob.append(matcher);
				}
				self.glob.append_char('}');
				self.glob
			}
			_ => {
				// TODO: Maybe make Matcher sortable,
				// even if it's a bad Ord implementation.
				self.options.sort_by(|a, b| {
					(!a.0.is_empty()).cmp(&!b.0.is_empty())
				});
				self.options.dedup();
				self.glob.append(super::Matcher::Any(self.options));
				self.glob
			}
		}
	}
	pub fn join(mut self) -> Glob {
		let mut first = true;
		self.glob.append_char('{');
		for option in self.options {
			if first {
				first = false;
			} else {
				self.glob.append_char(',')
			}
			self.glob.append_glob(option);
		}
		self.glob
	}
}
