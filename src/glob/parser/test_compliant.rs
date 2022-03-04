use crate::glob::{Glob, Matcher};

pub fn parse(glob: &str) -> Glob {
	let mut retval = Glob(vec![Matcher::Sep]);
	let mut chars = glob.chars().peekable();
	let mut stack = AltStack::new();
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
				(retval, chars) = super::parse_charclass(retval, chars);
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
				retval = stack.add_alt_and_pop(retval);
			}
			_ => retval.append_char(c)
		}
	}
	while !stack.is_empty() {
		retval = stack.add_alt_and_pop(retval);
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

	pub fn pop(&mut self) -> Option<Glob> {
		Some(self.0.pop()?.build())
	}

	pub fn add_alt_and_pop(&mut self, glob: Glob) -> Glob {
		match self.0.pop() {
			Some(mut builder) => {
				builder.add(glob);
				builder.build()
			}
			None => glob
		}
	}
}

pub struct AltBuilder {
	glob: Glob,
	options: Vec<Glob>,
	optional: bool,
}

impl AltBuilder {
	pub fn new(glob: Glob) -> AltBuilder {
		AltBuilder {
			glob,
			options: vec![],
			optional: false,
		}
	}
	pub fn add(&mut self, glob: Glob) {
		if !glob.0.is_empty() {
			self.options.push(glob);
		} else {
			self.optional = true;
		}
	}
	pub fn build(mut self) -> Glob {
		if self.optional {
			self.options.push(Glob(vec![]));
		}
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
				self.glob.append(super::Matcher::Any(self.options));
				self.glob
			}
		}
	}
}
