# ec4rs: EditorConfig For Rust

An
[EditorConfig](https://editorconfig.org/)
[core](https://editorconfig-specification.readthedocs.io/#terminology)
in safe Rust.
See [the Github repo](https://github.com/TheDaemoness/ec4rs)
for more information.

## Basic Example Usage

The most common usecase for `ec4rs` involves
determining how an editor/linter/etc should be configured
for a file at a given path.

The simplest way to load these is using [`properties_of`].
This function, if successful, will return a [`Properties`],
a map of config keys to values for a file at the provided path.
In order to get values for tab width and indent size that are compliant
with the standard, [`use_fallbacks`][Properties::use_fallbacks]
should be called before retrieving them.

From there, `Properties` offers several methods for retrieving values:

```
# #[cfg(feature = "ec4rs_glob")] {
// Read the EditorConfig files that would apply to a file at the given path.
let mut cfg = ec4rs::properties_of::<ec4rs::glob::Glob>("src/main.rs")
    .unwrap_or_default();
// Convenient access to ec4rs's property parsers.
use ec4rs::property::*;
// Use fallback values for tab width and/or indent size.
cfg.use_fallbacks();

// Let ec4rs do the parsing for you.
let indent_style: IndentStyle = cfg.get::<IndentStyle>()
    .unwrap_or(IndentStyle::Tabs);

// Get a string value, with a default.
// ec4rs has a basic string type designed to reduce allocations.
// See [`ec4rs::string::SharedString`] for more information.
let charset = cfg.get_raw::<Charset>()
    .cloned()
    .unwrap_or(ec4rs::string::SharedString::new_static("utf-8"));

// Parse a non-standard property.
let hard_wrap = cfg.get_raw_for_key("wildcard_import_limit")
    .unwrap_or_default()
    .parse::<usize>();
# }
```

## Glob Engine Support

The EditorConfig specification calls for some unusual glob features,
so for full specification compliance, `ec4rs_glob` exists as a
relatively-lightweight option. Support for it is controlled by the
`ec4rs_glob` feature flag, which is enabled by default.

However, if you don't need perfect spec compliance, there may be benefits
to using other glob engines. For instance:

- You may want stronger protections against absurdly complex patterns in
  untrusted EditorConfig files than the spec permits.
- You may already be using a different engine and don't want to redundantly
  import another one.
- You may have performance or resource usage requirements that `ec4rs_glob`
  cannot satisfy.

`ec4rs` has built-in support for [`globset`](https://crates.io/crates/globset)
which can be enabled with the `globset` feature flag. Otherwise,
you can bring your own glob engine by implementing the
[`Pattern`][crate::glob::Pattern] trait.

## Features

`bcp_47`:
Makes [`LanguageTag`][crate::property::LanguageTag] accept any well-formed
[Unicode BCP 47 locale identifier](https://unicode.org/reports/tr35/tr35.html#unicode_bcp47_locale_id)
using [`unic-locale`](https://docs.rs/unic-locale) for parsing and internal
representation.

`ec4rs_glob` (Default):
Enable support for an EditorConfig-compliant glob implementation.

`globset`:
Add support for [`globset`](https://docs.rs/globset/latest/globset/)
as an alternative glob implementation.
Note that `globset` patterns do not conform to the EditorConfig standard,
but it should be good enough for most real-world cases.

`track-source`: Allow [`SharedString`][crate::string::SharedString]
to store the file and line number it originates from.
[`ConfigParser`] will add this information where applicable.
