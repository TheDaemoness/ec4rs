#![allow(unused_imports)]
use crate::properties::{Properties, RawValue};

macro_rules! impls {
	($name:ident, $valuetype:ty, $val:pat_param, $cnv:expr) => {
		impl<'a> Iterator for $name<'a> {
			type Item = (&'a str, $valuetype);
			fn next(&mut self) -> Option<Self::Item> {
				loop {
					let pair = self.0.next()?;
					if pair.1.is_empty() {continue;}
					let (ref key, $val) = pair;
					break Some((key, $cnv))
				}
			}
			//TODO: Non-default implementations.
		}
		//TODO: PartialEq/Eq?
	}
}

/// An iterator over [Properties].
#[derive(Clone)]
pub struct Iter<'a> (pub(super) std::slice::Iter<'a, (String, String)>);

impls!{Iter, RawValue<'a>, ref val, RawValue::Unknown(val.as_str())}

/// An iterator over [Properties] that allows value mutation.
pub struct IterMut<'a> (pub(super) std::slice::IterMut<'a, (String, String)>);

impls!{IterMut, &'a mut String, ref mut val, val}
