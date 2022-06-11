# ec4rs: EditorConfig For Rust

An
[EditorConfig](https://editorconfig.org/)
[core](https://editorconfig-specification.readthedocs.io/#terminology) in Rust.

This library enables you to integrate EditorConfig support into any tools which may benefit from it,
such as code editors, formatters, and style linters.
It includes mechanisms for type-safe parsing of properties,
so that your tool doesn't have to do it itself.
It also exposes significant portions of its logic,
allowing you to use only the parts you need.

## Basic Example Usage

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
