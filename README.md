# ec4rs: EditorConfig For Rust
[![Check ec4rs](https://github.com/TheDaemoness/ec4rs/actions/workflows/check-lib.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/check-lib.yml)
[![crates.io](https://img.shields.io/crates/v/ec4rs.svg)](https://crates.io/crates/ec4rs)
[![API docs](https://docs.rs/ec4rs/badge.svg)](https://docs.rs/ec4rs)

An
[EditorConfig](https://editorconfig.org/)
[core](https://editorconfig-specification.readthedocs.io/#terminology)
in safe Rust.

This library enables you to integrate EditorConfig support
into any tools which may benefit from it,
such as code editors, formatters, and style linters.
It includes mechanisms for type-safe parsing of properties,
so that your tool doesn't have to do it itself.
It also exposes significant portions of its logic,
allowing you to use only the parts you need.

Name idea shamelessly stolen from [ec4j](https://github.com/ec4j/ec4j).
This library has minimal dependencies (only `std` at this time).

For example usage, see [the docs](https://docs.rs/ec4rs).

## Testing

[![Check Compliance](https://github.com/TheDaemoness/ec4rs/actions/workflows/check-ctest.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/check-ctest.yml)

The main repository for this library includes the EditorConfig
[core tests](https://github.com/editorconfig/editorconfig-core-test)
as a Git submodule. This library should pass all of these tests.
To run the test suite, run the following commands in a POSIX-like shell:

```bash
cargo build --package ec4rs_tools
git submodule update --init --recursive
cd tests
cmake -DEDITORCONFIG_CMD="$PWD/../target/debug/ec4rs-parse" .
ctest .
```

## Glob Library

[![Check ec4rs_glob](https://github.com/TheDaemoness/ec4rs/actions/workflows/check-glob.yml/badge.svg)](https://github.com/TheDaemoness/ec4rs/actions/workflows/check-glob.yml)

This repository also includes the [`ec4rs_glob`](/glob) library which is
primarily intended for internal use but can be used by other projects. See
[its README](/glob/README.md) for more information.

## License

**ec4rs**, **ec4rs_glob**, and **ec4rs_tools** are licensed under the
[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html)
with no `NOTICE` text.

Contributors submitting code changes must agree to the terms of the
[Developer Certificate of Origin (DCO)](https://developercertificate.org/)
to have their contributions accepted for inclusion.
A copy of the DCO may be found in `DCO.txt`.
Contributors should sign-off on their commits (see `git commit -s`)
to indicate explicit agreement.
