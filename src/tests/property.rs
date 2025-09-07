use crate::PropertyKey;

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
    contained!(SpellingLanguage);
    assert!(!STANDARD_KEYS.contains(&MaxLineLen::key())); // Not MaxLineLen
}

#[test]
fn spelling_language() {
    use crate::property::SpellingLanguage;
    use crate::string::SharedString;
    use crate::PropertyValue;
    // This is more testing language-tags than anything,
    // but for language-tags to be useful here,
    let testcase_en = SharedString::new_static("en");
    let parsed = match SpellingLanguage::from_shared_string(&testcase_en) {
        Ok(SpellingLanguage::Value(v)) => v,
        e => {
            let v = e.expect("parsing should succeed");
            panic!("unexpected value {v:?}");
        }
    };
    assert_eq!(parsed.primary_language(), &*testcase_en);
    let testcase_en_us = SharedString::new_static("en-US");
    let parsed = match SpellingLanguage::from_shared_string(&testcase_en_us) {
        Ok(SpellingLanguage::Value(v)) => v,
        e => {
            let v = e.expect("parsing should succeed");
            panic!("unexpected value {v:?}");
        }
    };
    assert_eq!(parsed.to_string(), &*testcase_en_us);
}
