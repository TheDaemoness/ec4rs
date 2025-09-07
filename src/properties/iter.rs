#![allow(unused_imports)]
use crate::properties::Properties;
use crate::string::SharedString;

macro_rules! impls {
    ($name:ident, $valuetype:ty) => {
        impl<'a> Iterator for $name<'a> {
            type Item = (&'a str, $valuetype);
            fn next(&mut self) -> Option<Self::Item> {
                let pair = self.0.next()?;
                let (ref key, val) = pair;
                Some((key, val))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }
        impl<'a> DoubleEndedIterator for $name<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                let pair = self.0.next_back()?;
                let (ref key, val) = pair;
                Some((key, val))
            }
        }
        impl<'a> std::iter::FusedIterator for $name<'a> {}
        //TODO: PartialEq/Eq?
    };
}

/// An iterator over [`Properties`].
#[derive(Clone)]
pub struct Iter<'a>(pub(super) std::slice::Iter<'a, (SharedString, SharedString)>);

impls! {Iter, &'a SharedString}

/// An iterator over [`Properties`] that allows value mutation.
pub struct IterMut<'a>(pub(super) std::slice::IterMut<'a, (SharedString, SharedString)>);

impls! {IterMut, &'a mut SharedString}
