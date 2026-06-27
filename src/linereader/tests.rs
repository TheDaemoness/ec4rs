use crate::linereader::*;
use crate::ParseError;

fn test_lines(lines: &[(&'static str, Line<'static>)]) {
    for (line, expected) in lines {
        assert_eq!(parse_line(line).unwrap(), *expected)
    }
}

#[test]
fn valid_props() {
    use Line::Pair;
    test_lines(&[
        ("foo=bar", Pair("foo", "bar")),
        ("Foo=Bar", Pair("Foo", "Bar")),
        ("foo = bar", Pair("foo", "bar")),
        ("  foo   =   bar  ", Pair("foo", "bar")),
        ("foo=bar=baz", Pair("foo", "bar=baz")),
        ("  foo =  bar = baz  ", Pair("foo", "bar = baz")),
        ("foo = bar #baz", Pair("foo", "bar #baz")),
        ("foo = [bar]", Pair("foo", "[bar]")),
        ("foo =", Pair("foo", "")),
        ("foo = ", Pair("foo", "")),
    ])
}

#[test]
fn valid_sections() {
    use Line::Section;
    test_lines(&[
        ("[foo]", Section("foo")),
        ("[[foo]]", Section("[foo]")),
        ("[ foo ]", Section(" foo ")),
        ("[[]", Section("[")),
        ("[]]", Section("]")),
        ("[][]", Section("][")),
        ("[Foo]", Section("Foo")),
        (" [foo] ", Section("foo")),
        ("[a=b]", Section("a=b")),
        ("[#foo]", Section("#foo")),
        ("[foo] #comment", Section("foo")),
        ("[foo] ;comment", Section("foo")),
    ])
}

#[test]
fn valid_nothing() {
    use Line::Nothing;
    test_lines(&[
        ("\t", Nothing),
        ("\r", Nothing),
        ("", Nothing),
        ("   ", Nothing),
        (";comment", Nothing),
        ("#comment", Nothing),
        ("  # comment", Nothing),
        ("# [section]", Nothing),
        ("# foo=bar", Nothing),
    ])
}

#[test]
fn invalid_lines() {
    // Invalid lines.
    let lines = [
        "[close",
        "open]",
        "][",
        "nonproperty",
        "=",
        "  = nokey",
        "// C++-style comment",
    ];
    for line in lines {
        assert!(matches!(
            parse_line(line).unwrap_err(),
            ParseError::InvalidLine
        ))
    }
}

#[test]
fn invalid_sections() {
    assert!(matches!(
        parse_line("[]").unwrap_err(),
        ParseError::InvalidSection(None)
    ));
    let pairs = [
        ("[foo][", "["),
        ("[foo] bar", " bar"),
        ("[foo] bar ; baz", " bar ; baz"),
        ("[foo]=bar", "=bar"),
    ];
    for (line, expected) in pairs {
        let e = match parse_line(line) {
            Err(ParseError::InvalidSection(Some(v))) => v,
            e => panic!("unexpected result {e:?}"),
        };
        assert_eq!(&*e, expected, "mismatch in expected error data");
    }
}
