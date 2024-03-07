#![allow(unused_imports)]
use crate::properties::{Properties, RawValue};

macro_rules! impls {
    ($name:ident, $valuetype:ty) => {
        impl<'a> Iterator for $name<'a> {
            type Item = (&'a str, $valuetype);
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    let pair = self.0.next()?;
                    if pair.1.is_unset() {
                        continue;
                    }
                    let (ref key, val) = pair;
                    break Some((key, val));
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                // TODO: Exact size when empty pairs are disallowed from Properties.
                (0, self.0.size_hint().1)
            }
        }
        impl<'a> DoubleEndedIterator for $name<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                loop {
                    let pair = self.0.next_back()?;
                    if pair.1.is_unset() {
                        continue;
                    }
                    let (ref key, val) = pair;
                    break Some((key, val));
                }
            }
        }
        impl<'a> std::iter::FusedIterator for $name<'a> {}
        //TODO: PartialEq/Eq?
    };
}

/// An iterator over [`Properties`].
#[derive(Clone)]
pub struct Iter<'a>(pub(super) std::slice::Iter<'a, (String, RawValue)>);

impls! {Iter, &'a RawValue}

/// An iterator over [`Properties`] that allows value mutation.
pub struct IterMut<'a>(pub(super) std::slice::IterMut<'a, (String, RawValue)>);

impls! {IterMut, &'a mut RawValue}
