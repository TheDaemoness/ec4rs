use super::LanguageTag;
use crate::property::*;

macro_rules! test_each {
    (for $prop:ident in ($($item:path),+) $test:block) => {{
        $({
            type $prop = $item;
            $test;
        })+
    }};
    (for $prop:ident in standard $test:block) => {
        test_each!(for $prop in (
            IndentStyle,
            IndentSize,
            TabWidth,
            EndOfLine,
            Charset,
            TrimTrailingWs,
            FinalNewline,
            SpellingLanguage
        ) $test)
    };
    (for $prop:ident in all $test:block) => {
        test_each!(for $prop in (
            IndentStyle,
            IndentSize,
            TabWidth,
            EndOfLine,
            Charset,
            TrimTrailingWs,
            FinalNewline,
            SpellingLanguage,
            MaxLineLen
        ) $test )
    }
}

#[test]
fn standard_keys_matches() {
    assert_eq!(STANDARD_KEYS.len(), 8);
    test_each!(for Prop in standard {
        assert!(
            STANDARD_KEYS.contains(&Prop::key()),
            "STANDARD_KEYS is missing {}",
            Prop::key()
        )
    });
    assert!(!STANDARD_KEYS.contains(&MaxLineLen::key())); // Not MaxLineLen
}

#[test]
fn spelling_language() {
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

#[test]
fn language_tag_rejects_root() {
    LanguageTag::try_from("root").expect_err("Language name 'root' not rejected");
}

#[test]
fn unset_parses() {
    test_each!(for Prop in all {
        use crate::string::ToSharedString;
        // crate::traits::test_reparse exists but shouldn't be used here
        // so that we can ensure that the parsed value is "unset".
        assert_eq!("unset".parse::<Prop>().ok(), Some(Prop::Unset));
        assert_eq!(Prop::Unset.to_shared_string(), "unset".into());
        // Test case-insensitivity here for lack of a better place to test it.
        assert_eq!("UnSeT".parse::<Prop>().ok(), Some(Prop::Unset));
    })
}

#[test]
fn bogus_values() {
    test_each!(for Prop in all {
        "unaccep7abl3arbi7rag3"
            .parse::<Prop>()
            .expect_err("Nonsense value not rejected");
        // Negative values are not currently allowed in any key-value pair.
        "-1".parse::<Prop>()
            .expect_err("Negative value not rejected");
    })
}
