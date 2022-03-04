use super::Glob;

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
		if !glob.0.is_empty() {
			self.options.push(glob);
		}
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
				self.glob.append(super::Matcher::Any(self.options));
				self.glob
			}
		}
	}
}
