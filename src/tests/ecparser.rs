fn validate<'a>(
    text: &str,
    should_be_root: bool,
    expected: impl IntoIterator<Item = &'a [(&'a str, &'a str)]>,
) {
    let mut parser =
        crate::ConfigParser::new(text.as_bytes()).expect("Should have created the parser");
    assert_eq!(parser.is_root, should_be_root);
    for section_expected in expected {
        let section = parser.next().unwrap().unwrap();
        let mut iter = section.props().iter().map(|(k, v)| (k, v.into_str()));
        for (key, value) in section_expected {
            assert_eq!(iter.next(), Some((*key, *value)))
        }
        assert!(iter.next().is_none());
    }
    assert!(parser.next().is_none());
}

macro_rules! expect {
	[$([$(($key:literal, $value:literal)),*]),*] => {
		[$(&[$(($key, $value)),*][..]),*]
	}
}

#[test]
fn empty() {
    validate("", false, expect![]);
}

#[test]
fn prelude() {
    validate("root = true\nroot = false", false, expect![]);
    validate("root = true", true, expect![]);
    validate("Root = True", true, expect![]);
    validate("# hello world", false, expect![]);
}

#[test]
fn prelude_unknown() {
    validate("foo = bar", false, expect![]);
    validate("foo = bar\nroot = true", true, expect![]);
}

#[test]
fn sections_empty() {
    validate("[foo]", false, expect![[]]);
    validate("[foo]\n[bar]", false, expect![[], []]);
}

#[test]
fn sections() {
    validate(
        "[foo]\nbk=bv\nak=av",
        false,
        expect![[("bk", "bv"), ("ak", "av")]],
    );
    validate(
        "[foo]\nbk=bv\n[bar]\nak=av",
        false,
        expect![[("bk", "bv")], [("ak", "av")]],
    );
    validate(
        "[foo]\nk=a\n[bar]\nk=b",
        false,
        expect![[("k", "a")], [("k", "b")]],
    );
}

#[test]
fn trailing_newline() {
    validate("[foo]\nbar=baz\n", false, expect![[("bar", "baz")]]);
    validate("[foo]\nbar=baz\n\n", false, expect![[("bar", "baz")]]);
}

#[test]
fn section_with_comment_after_it() {
    validate(
        "[/*] # ignore this comment\nk=v",
        false,
        expect![[("k", "v")]],
    );
}
