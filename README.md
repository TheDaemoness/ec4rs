# ec4rs: EditorConfig For Rust
[![CI](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/ci.yml)

An `std`-only EditorConfig library in pure safe Rust.

Name idea shamelessly stolen from [ec4j](https://github.com/ec4j/ec4j).

_Status note:
This project is incomplete, but is in active development.
It implements a subset of EditorConfig
that should work in most real-world cases.
Its API may change somewhat prior to the 1.0.0 release._

Current limitations:
* Only one version of EditorConfig is supported.
* `{}` are not supported in glob patterns.
* Error handling is inflexible.
* There is no type for storing entire parsed config files in memory.
