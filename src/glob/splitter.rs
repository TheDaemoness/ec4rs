// Problem.
// OsStr cannot be cast to &[u8] on Windows.
// On Unixes and WASM it's fine.

#[cfg(target_family = "unix")]
mod cnv {
	use std::ffi::OsStr;
	pub fn to_bytes(s: &OsStr) -> Option<&[u8]> {
		use std::os::unix::ffi::OsStrExt;
		Some(s.as_bytes())
	}
	pub fn to_path(s: &[u8]) -> &std::path::Path {
		use std::os::unix::ffi::OsStrExt;
		OsStr::from_bytes(s).as_ref()
	}
}


#[cfg(target_os = "wasi")]
mod cnv {
	use std::ffi::OsStr;
	pub fn to_bytes(s: &OsStr) -> Option<&[u8]> {
		use std::os::wasi::ffi::OsStrExt;
		Some(s.as_bytes())
	}
	pub fn to_path(s: &[u8]) -> &std::path::Path {
		use std::os::wasi::ffi::OsStrExt;
	}
}

#[cfg(all(
	not(target_family = "unix"),
	not(target_os = "wasi")
))]
mod cnv {
	use std::ffi::OsStr;
	pub fn to_bytes(s: &OsStr) -> Option<&[u8]> {
		s.to_str().map(|s| s.as_ref())
	}
	pub fn to_path(s: &[u8]) -> &std::path::Path {
		std::str::from_utf8(s).unwrap().as_ref()
	}
}

#[derive(Clone)]
pub struct Splitter<'a> {
	iter: std::path::Components<'a>,
	part: &'a [u8]
}

impl<'a> Splitter<'a> {
	pub fn new(path: &std::path::Path) -> Option<Splitter<'_>> {
		Splitter {
			iter: path.components(),
			part: "".as_bytes(),
		}.next()
	}


	pub fn next(mut self) -> Option<Splitter<'a>> {
		use std::path::Component::*;
		self.part = match self.iter.next_back()? {
			Normal(p) => cnv::to_bytes(p)?,
			ParentDir => "..".as_bytes(),
			_ => "".as_bytes(),
		};
		Some(self)
	}

	pub fn match_any(mut self, path_sep: bool) -> Option<Splitter<'a>> {
		if !self.part.is_empty() {
			self.part = self.part.split_last().unwrap().1;
			Some(self)
		} else {
			path_sep.then(|| self.match_sep()).flatten()
		}
	}

	pub fn next_char(mut self) -> Option<(Splitter<'a>, char)> {
		// TODO: Don't recheck the part for valid unicode each time.
		if let Ok(s) = std::str::from_utf8(self.part) {
			if let Some((idx, c)) = s.char_indices().next_back() {
				self.part = s.split_at(idx).0.as_bytes();
				return Some((self, c))
			} else {
				return Some((self.next()?, '/'))
			}
		}
		None
	}

	pub fn match_sep(self) -> Option<Splitter<'a>> {
		if self.part.is_empty() {
			self.next()
		} else {
			None
		}
	}

	pub fn match_suffix(mut self, suffix: &str) -> Option<Splitter<'a>> {
		if let Some(rest) = self.part.strip_suffix(suffix.as_bytes()) {
			self.part = rest;
			Some(self)
		} else {
			None
		}
	}
}
