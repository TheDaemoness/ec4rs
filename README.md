# ec4rs: EditorConfig For Rust
[![CI](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ec4rs.svg)](https://crates.io/crates/ec4rs)
[![API docs](https://docs.rs/ec4rs/badge.svg)](https://docs.rs/ec4rs)

An
[EditorConfig](https://editorconfig.org/)
[core](https://editorconfig-specification.readthedocs.io/#terminology) in Rust.

This library enables you to integrate EditorConfig support into any tools which may benefit from it,
such as code editors, formatters, and style linters.
It includes mechanisms for type-safe parsing of properties,
so that your tool doesn't have to do it itself.
It also exposes significant portions of its logic,
allowing you to use only the parts you need.

Name idea shamelessly stolen from [ec4j](https://github.com/ec4j/ec4j).
This library has minimal dependencies (only `std` at this time),

For more information, see [the docs](https://docs.rs/ec4rs).
