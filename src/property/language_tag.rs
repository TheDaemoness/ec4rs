use super::UnknownValueError;

// TODO: Use std::ascii::Char when it stabilizes.

/// The subset of BCP 47 language tags permitted by the EditorConfig standard.
///
/// This type doesn't implement [`PropertyValue`][crate::PropertyValue].
/// See [`SpellingLanguage`][super::SpellingLanguage].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LanguageTag([u8; 4]);

impl LanguageTag {
    /// Attempt to parse `self` from the provided string.
    pub fn try_from(string: impl AsRef<str>) -> Result<Self, UnknownValueError> {
        match string.as_ref().as_bytes() {
            [a, b] => (a.is_ascii_alphabetic() && b.is_ascii_alphabetic())
                .then(|| Self([a.to_ascii_lowercase(), b.to_ascii_lowercase(), 0, 0])),
            [a, b, b'-', c, d] => (a.is_ascii_alphabetic()
                && b.is_ascii_alphabetic()
                && c.is_ascii_alphabetic()
                && d.is_ascii_alphabetic())
            .then(|| {
                Self([
                    a.to_ascii_lowercase(),
                    b.to_ascii_lowercase(),
                    c.to_ascii_uppercase(),
                    d.to_ascii_uppercase(),
                ])
            }),
            _ => None,
        }
        .ok_or(UnknownValueError)
    }
    /// Returns the language subtag, e.g. the "en" in "en-US".
    pub fn primary_language(&self) -> &str {
        #[allow(clippy::missing_panics_doc)]
        std::str::from_utf8(&self.0[0..2]).expect("Non-UTF-8 bytes in LanguageTag")
    }
    /// Returns the region subtag, if any, e.g. the "US" in "en-US".
    pub fn region(&self) -> Option<&str> {
        let slice = &self.0[2..4];
        #[allow(clippy::missing_panics_doc)]
        (*slice != [0, 0])
            .then(|| std::str::from_utf8(slice).expect("Non-UTF-8 bytes in LanguageTag"))
    }
}

impl std::fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(region) = self.region() {
            write!(f, "{}-{}", self.primary_language(), region)
        } else {
            write!(f, "{}", self.primary_language())
        }
    }
}

impl std::str::FromStr for LanguageTag {
    type Err = UnknownValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
