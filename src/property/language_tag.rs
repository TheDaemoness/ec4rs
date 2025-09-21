use super::UnknownValueError;

// TODO: Use std::ascii::Char when it stabilizes.

/// The subset of BCP 47 language tags permitted by the EditorConfig standard.
///
/// If the `bcp-47` feature flag is enabled, this type becomes a newtype
/// around [`Locale`][unic_locale::Locale] and can accept any well-formed
/// [Unicode BCP 47 locale identifier](https://unicode.org/reports/tr35/tr35.html#unicode_bcp47_locale_id)
///
/// This type doesn't implement [`PropertyValue`][crate::PropertyValue].
/// See [`SpellingLanguage`][super::SpellingLanguage].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LanguageTag(
    #[cfg(not(feature = "bcp-47"))]
    [u8; 4],
    #[cfg(feature = "bcp-47")]
    unic_locale::Locale
);

impl LanguageTag {
    /// Attempt to parse `self` from the provided string.
    pub fn try_from(string: impl AsRef<str>) -> Result<Self, UnknownValueError> {
        #[cfg(not(feature = "bcp-47"))]
        {
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
        #[cfg(feature = "bcp-47")]
        {
            let parsed: unic_locale::Locale = string.as_ref().parse().or(Err(UnknownValueError))?;
            // No need to do anything special to reject the "root" language.
            Ok(LanguageTag(parsed))
        }
    }
    /// Returns the language subtag, e.g. the "en" in "en-US".
    pub fn primary_language(&self) -> &str {
        #[cfg(not(feature = "bcp-47"))]
        {
            #[allow(clippy::missing_panics_doc)]
            std::str::from_utf8(&self.0[0..2]).expect("Non-UTF-8 bytes in LanguageTag")
        }
        #[cfg(feature = "bcp-47")]
        {
            self.0.id.language.as_str()
        }
    }
    /// Returns the region subtag, if any, e.g. the "US" in "en-US".
    pub fn region(&self) -> Option<&str> {
        #[cfg(not(feature = "bcp-47"))]
        {
            let slice = &self.0[2..4];
            #[allow(clippy::missing_panics_doc)]
            (*slice != [0, 0])
                .then(|| std::str::from_utf8(slice).expect("Non-UTF-8 bytes in LanguageTag"))
        }
        #[cfg(feature = "bcp-47")]
        {
            self.0.id.region.as_ref().map(unic_locale::subtags::Region::as_str)
        }
    }
    /// Returns the script subtag, if any, e.g. the "Latn" in "en-Latn-US".
    ///
    /// Without the `bcp-47` feature, this method will always return `None`
    /// because the parser rejects inputs that would contain a script subtag.
    pub fn script(&self) -> Option<&str> {
        #[cfg(not(feature = "bcp-47"))]
        {
            None
        }
        #[cfg(feature = "bcp-47")]
        {
            self.0.id.script.as_ref().map(unic_locale::subtags::Script::as_str)
        }
    }
    #[cfg(feature = "bcp-47")]
    /// Returns a reference to the inner [`unic_locale::Locale`].
    pub fn as_inner(&self) -> &unic_locale::Locale {
        &self.0
    }
    #[cfg(feature = "bcp-47")]
    /// Returns a mutable reference to the inner [`unic_locale::Locale`].
    pub fn as_inner_mut(&mut self) -> &mut unic_locale::Locale {
        &mut self.0
    }
    #[cfg(feature = "bcp-47")]
    /// Returns the inner [`unic_locale::Locale`].
    pub fn into_inner(self) -> unic_locale::Locale {
        self.0
    }
}

#[cfg(feature = "bcp-47")]
impl From<unic_locale::Locale> for LanguageTag {
    fn from(value: unic_locale::Locale) -> Self {
        LanguageTag(value)
    }
}

impl std::fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "bcp-47"))]
        if let Some(region) = self.region() {
            write!(f, "{}-{}", self.primary_language(), region)
        } else {
            write!(f, "{}", self.primary_language())
        }
        #[cfg(feature = "bcp-47")]
        {
            self.0.fmt(f)
        }
    }
}

impl std::str::FromStr for LanguageTag {
    type Err = UnknownValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
