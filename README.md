# ec4rs: EditorConfig For Rust
[![CI](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ec4rs.svg)](https://crates.io/crates/ec4rs)
[![API docs](https://docs.rs/ec4rs/badge.svg)](https://docs.rs/ec4rs)

An EditorConfig core in Rust.

Name idea shamelessly stolen from [ec4j](https://github.com/ec4j/ec4j).

This library has minimal dependencies (at this time, only `std`),
and gives you as much or as little control as you want to build
tools that work with EditorConfig files.
It also includes mechanisms for type-safe parsing of properties,
so that your tool doesn't have to do it itself.

For more information, see [the docs](https://docs.rs/ec4rs).
