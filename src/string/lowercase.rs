use std::borrow::Cow;

pub(crate) fn into_lowercase(string: &str) -> Cow<'_, str> {
    let mut lcit = string.chars().flat_map(char::to_lowercase);
    let mut idx = 0usize;
    let mut lc_char = Option::<char>::None;
    for c in string.chars() {
        let Some(lc) = lcit.next() else {
            return Cow::Borrowed(string.split_at(idx).0);
        };
        if c == lc {
            idx += c.len_utf8();
        } else {
            lc_char = Some(lc);
            break;
        }
    }
    match lc_char.or_else(|| lcit.next()) {
        None => Cow::Borrowed(string),
        Some(c) => {
            let mut retval = String::with_capacity(string.len() + 1);
            retval.push_str(string.split_at(idx).0);
            retval.push(c);
            retval.extend(lcit);
            Cow::Owned(retval)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::into_lowercase;

    #[test]
    fn noops() {
        assert_eq!(into_lowercase(""), "");
        assert_eq!(into_lowercase("a"), "a");
        assert_eq!(into_lowercase("_"), "_");
        assert_eq!(into_lowercase("ab"), "ab");
        assert_eq!(into_lowercase("__"), "__");
        assert_eq!(into_lowercase("a_"), "a_");
        assert_eq!(into_lowercase("_b"), "_b");
        assert_eq!(into_lowercase("a_c"), "a_c");
        assert_eq!(into_lowercase("ab_"), "ab_");
        assert_eq!(into_lowercase("_bc"), "_bc");
    }
    #[test]
    fn basic() {
        assert_eq!(into_lowercase("A"), "a");
        assert_eq!(into_lowercase("Ab"), "ab");
        assert_eq!(into_lowercase("aB"), "ab");
        assert_eq!(into_lowercase("Abc"), "abc");
        assert_eq!(into_lowercase("aBc"), "abc");
        assert_eq!(into_lowercase("abC"), "abc");
    }
}
