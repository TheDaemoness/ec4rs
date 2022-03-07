# ec4rs: EditorConfig For Rust
[![CI](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml)

An EditorConfig core in Rust.

Name idea shamelessly stolen from [ec4j](https://github.com/ec4j/ec4j).

This library has minimal dependencies (at this time, only `std`),
and gives you as much or as little control as you want to build
tools that work with EditorConfig files.
It also includes mechanisms for type-safe parsing of properties,
so that your tool doesn't have to do it itself.

## Example Usage

```
// Read the EditorConfig files that would apply to a file at the given path.
let mut cfg = ec4rs::config_for("src/main.rs").unwrap_or_default();
// Convenient access to ec4rs's property parsers.
use ec4rs::property::*;
// Use fallback values for tab width and/or indent size.
cfg.use_fallbacks();
// Let ec4rs do the parsing for you.
let indent_style: IndentStyle = cfg.get::<IndentStyle>().unwrap_or(IndentStyle::Tabs);
// Get a string value, with a default.
// filter_unset handles the special value "unset" for you.
let charset: &str = cfg.get_raw::<Charset>().filter_unset().into_result().unwrap_or("utf-8");
// Parse a non-standard property, lowercasing the value before doing so.
let hard_wrap: Option<usize> = cfg.get_raw_for_key("max_line_length").parse::<usize, true>().ok();
```
