use super::alt::AltStack;
use crate::{Glob, Matcher};

pub fn parse(glob: &str) -> Glob {
    let mut retval = Glob(vec![]);
    let mut stack = AltStack::new();
    for segment in glob.split('/') {
        retval.append_escaped('/');
        let mut chars = segment.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(escaped) = chars.next() {
                        retval.append_escaped(escaped);
                    }
                }
                '?' => retval.push(Matcher::AnyChar),
                '*' => retval.push(Matcher::AnySeq(matches!(chars.peek(), Some('*')))),
                '[' => {
                    let (retval_n, chars_n) = super::charclass::parse(retval, chars);
                    retval = retval_n;
                    chars = chars_n;
                }
                '{' => {
                    if let Some((a, b, chars_new)) = super::numrange::parse(chars.clone()) {
                        chars = chars_new;
                        retval.push(Matcher::Range(
                            // Reading the spec strictly,
                            // a compliant implementation must handle cases where
                            // the left integer is greater than the right integer.
                            std::cmp::min(a, b),
                            std::cmp::max(a, b),
                        ));
                    } else {
                        stack.push(retval);
                        retval = Glob(vec![]);
                    }
                }
                ',' => {
                    if let Some(rejected) = stack.add_alt(retval) {
                        retval = rejected;
                        retval.append_escaped(',');
                    } else {
                        retval = Glob(vec![]);
                    }
                }
                '}' => {
                    let (retval_n, add_brace) = stack.add_alt_and_pop(retval);
                    retval = retval_n;
                    if add_brace {
                        retval.append_escaped('}');
                    }
                }
                _ => retval.append_escaped(c),
            }
        }
    }
    loop {
        let (retval_n, is_empty) = stack.join_and_pop(retval);
        retval = retval_n;
        if is_empty {
            break;
        }
    }
    if glob.contains("/") {
        *retval.0.first_mut().unwrap() = Matcher::End;
    }
    if let Some(Matcher::Sep) = retval.0.last() {
        retval.push(Matcher::AnySeq(false));
    }
    retval
}
