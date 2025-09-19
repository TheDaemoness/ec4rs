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
// ec4rs has a string type designed for immutability and minimal allocations.
let charset = cfg.get_raw::<Charset>()
    .cloned()
    .unwrap_or(ec4rs::string::SharedString::new_static("utf-8"));

// Parse a non-standard property.
let hard_wrap = cfg.get_raw_for_key("max_line_length")
    .unwrap_or_default()
    .parse::<usize>();
```

## Features

`ec4rs_glob` (Default):
Enable support for an EditorConfig-compliant glob implementation.

`globset`:
Add support for [`globset`](https://docs.rs/globset/latest/globset/)
as an alternative glob implementation.
Note that `globset` patterns do not conform to the EditorConfig standard,
but it should be good enough for most real-world cases.

`language-tags`: NYI for 2.0.

`track-source`: Allow [`SharedString`][crate::string::SharedString]
to store the file and line number it originates from.
[`ConfigParser`] will add this information where applicable.
