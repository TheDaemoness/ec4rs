use crate::Properties;

static BASIC_KEYS: [&'static str; 4] = ["2", "3", "0", "1"];
static ALT_VALUES: [&'static str; 4] = ["a", "b", "c", "d"];

fn zip_self() -> impl Iterator<Item = (&'static str, &'static str)> {
	BASIC_KEYS.iter().cloned().zip(BASIC_KEYS.iter().cloned())
}

fn zip_alts() -> impl Iterator<Item = (&'static str, &'static str)> {
	BASIC_KEYS.iter().cloned().zip(ALT_VALUES.iter().cloned())
}

fn test_basic_keys(props: &Properties) {
	for s in BASIC_KEYS {
		// Test mapping correctness using get.
		assert_eq!(props.get_raw_for_key(s).value(), Some(s))
	}
	// Ensure that they keys are returned in order.
	assert!(props.iter_raw().map(|k| k.0).eq(BASIC_KEYS.iter().cloned()))
}

#[test]
fn from_iter() {
	let props: Properties = zip_self().collect();
	test_basic_keys(&props);
}

#[test]
fn insert() {
	let mut props = Properties::new();
	for s in BASIC_KEYS {
		props.insert_raw_for_key(s, s);
	}
	test_basic_keys(&props);
}

#[test]
fn insert_replacing() {
	let mut props: Properties = zip_alts().collect();
	for (k, v) in zip_alts() {
		let old = props.get_raw_for_key(k).value().expect("missing pair").to_owned();
		assert_eq!(old, v);
		props.insert_raw_for_key(k, k);
	}
	test_basic_keys(&props);
}

#[test]
fn try_insert() {
	let mut props = Properties::new();
	for s in BASIC_KEYS {
		assert!(props.try_insert_raw_for_key(s, s).is_ok());
	}
	test_basic_keys(&props);
}

#[test]
fn try_insert_replacing() {
	let mut props: Properties = zip_self().collect();
	for (k, v) in zip_alts() {
		assert_eq!(
			props
				.try_insert_raw_for_key(k, k)
				.expect_err("try_insert wrongly returns Ok for same value"),
			k
		);
		assert_eq!(
			props
				.try_insert_raw_for_key(k, v)
				.expect_err("try_insert wrongly returns Ok for update"),
			k
		);
	}
}
