pub fn test<'a,'b>(
	pattern: &str,
	valid:   impl IntoIterator<Item = &'a str>,
	invalid: impl IntoIterator<Item = &'b str>) {
	let glob = crate::glob::Glob::parse(pattern).unwrap();
	for path in valid {
		assert!(glob.matches(path.as_ref()), "`{path}` didn't match pattern `{pattern}`")
	}
	for path in invalid {
		assert!(!glob.matches(path.as_ref()), "`{path}` wrongly matched pattern `{pattern}`")
	}
}

#[test]
fn basic() {
	test(
		"foo",
		["/foo", "./foo", "/bar/foo"],
		["foo", "/foobar", "/barfoo"]
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
