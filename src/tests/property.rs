#[test]
fn standard_keys_matches() {
	use crate::property::*;
	macro_rules! contained {
		($prop:ident) => {
			assert!(
				STANDARD_KEYS.contains(&$prop::key()),
				"STANDARD_KEYS is missing {}",
				$prop::key()
			)
		};
	}
	contained!(IndentStyle);
	contained!(IndentSize);
	contained!(TabWidth);
	contained!(EndOfLine);
	contained!(Charset);
	contained!(TrimTrailingWs);
	contained!(FinalNewline);
	assert!(!STANDARD_KEYS.contains(&MaxLineLen::key())); // Not MaxLineLen
}
