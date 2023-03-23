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

The simplest way to load these is using [`properties_of`], like so:

```
// Read the EditorConfig files that would apply to a file at the given path.
let mut cfg = ec4rs::properties_of("src/main.rs").unwrap_or_default();
// Convenient access to ec4rs's property parsers.
use ec4rs::property::*;
// Use fallback values for tab width and/or indent size.
cfg.use_fallbacks();

// Let ec4rs do the parsing for you.
let indent_style: IndentStyle = cfg.get::<IndentStyle>()
    .unwrap_or(IndentStyle::Tabs);

// Get a string value, with a default.
let charset: &str = cfg.get_raw::<Charset>()
    .filter_unset() // Handle the special "unset" value.
    .into_option()
    .unwrap_or("utf-8");

// Parse a non-standard property.
let hard_wrap = cfg.get_raw_for_key("max_line_length")
    .into_str()
    .parse::<usize>();
```
