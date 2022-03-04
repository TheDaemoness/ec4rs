pub fn test<'a,'b>(
	pattern: &str,
	valid:   impl IntoIterator<Item = &'a str>,
	invalid: impl IntoIterator<Item = &'b str>) {
	use crate::glob::{parse, matches};
	let glob = parse(pattern).unwrap();
	for path in valid {
		assert!(
			matches(&glob, path.as_ref()),
			"`{path}` didn't match pattern `{pattern}`; chain: {:?}", glob
		)
	}
	for path in invalid {
		assert!(
			!matches(&glob, path.as_ref()),
			"`{path}` wrongly matched pattern `{pattern}`; chain {:?}", glob
		)
	}
}

#[test]
fn basic() {
	test(
		"foo",
		["/foo", "./foo", "/bar/foo"],
		["foo", "/foobar", "/barfoo"]
	);
	test(
		"foo,bar",
		["/foo,bar"],
		["/foo", "/bar"]
	);
}

#[test]
fn path() {
	test(
		"bar/foo",
		["/bar/foo", "/baz/bar/foo", "/bar//foo"],
		["/bar/foo/baz"]
	);
}

#[test]
fn star() {
	test(
		"*.foo",
		["/a.foo", "/b.foo", "/ab.foo", "/bar/abc.foo", "/.foo"],
		[]
	);
	test(
		"bar*.foo",
		["/bar.foo", "/barab.foo", "/baz/bara.foo", "/bar.foo"],
		["/bar/.foo"]
	);
}

#[test]
fn charclass_basic() {
	test(
		"[a]",
		["/a"],
		["/aa", "/b"]
	);
	test(
		"[a][b]",
		["/ab"],
		["/aa", "/ba", "/cab"]
	);
	test(
		"[ab]",
		["/a", "/b"],
		["/ab"]
	);
	test(
		"[!ab]",
		["/c"],
		["/a", "/b", "/ab", "/ac"]
	)
}

#[test]
fn charclass_slash() {
	// See the brackets_slash_inside tests.
	test(
		"a[b/]c",
		["/a[b/]c"],
		["/abc", "/a/c"]
	);
}

#[test]
fn charclass_range() {
	test(
		"[a-c]",
		["/a", "/b", "/c"],
		["/d"]
	);
	test(
		"[-]",
		["/-"],
		["/"]
	);
	test(
		"[-a]",
		["/-", "/a"],
		[]
	);
	test(
		"[a-]",
		["/-", "/a"],
		[]
	);
}

#[test]
fn charclass_escape() {
	test(
		"[\\]a]",
		["/]", "/a"],
		[]
	);
	test(
		"[a\\-c]",
		["/a", "/-", "/c"],
		["/b"]
	);
	test(
		"[[-\\]^]",
		["/[", "/]", "/^"],
		[]
	);
}

#[test]
fn numrange() {
	test(
		"{8..11}",
		["/8", "/9", "/10", "/11"],
		["/12", "/1", "/01"]
	);
	test(
		"{-3..-1}",
		["/-3", "/-2", "/-1"],
		["/0", "/1"]
	);
	test(
		"{2..-1}",
		["/2", "/1", "/0", "/-1"],
		["/-2"]
	);
}

#[test]
fn alt() {
	test(
		"{foo}",
		["/foo"],
		["/{foo}"]
	);
	test(
		"{foo}.bar",
		["/foo.bar"],
		["/foo", "/{foo}.bar"]
	);
	test(
		"{foo,bar}",
		["/foo", "/bar"],
		["/foo,bar", "/foobar"]
	);
}
#[test]
fn alt_nested() {
	test(
		"{a{bc,cd},e}",
		["/abc","/acd", "/e"],
		["/cd"]
	);
}
