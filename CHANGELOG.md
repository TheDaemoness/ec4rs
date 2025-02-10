# 1.x

## 1.2.0 (Next)

- Added feature `track-source` to track where any given value came from.
- Added `-0Hl` flags to `ec4rs-parse` for displaying value sources.
- Added `RawValue::to_lowercase`.
- Implemented `Display` for `RawValue`.
- Changed `ec4rs-parse` to support empty values for compliance with
EditorConfig `0.17.2`.
- Fixed fallbacks adding an empty value for `indent_size`.
- Fixed `Properties::iter` and `Properties::iter_mut` not returning
pairs with empty values when `allow-empty-values` is enabled.

## 1.1.1 (2024-08-29)

- Update testing instructions to work with the latest versions of cmake+ctest.
- Fix `/*` matching too broadly (#12).

## 1.1.0 (2024-03-26)

- Added optional `spelling_language` parsing for EditorConfig `0.16.0`.
This adds an optional dependency on the widely-used `language-tags` crate
to parse a useful superset of the values allowed by the spec.
- Added feature `allow-empty-values` to allow empty key-value pairs (#7).
Added to opt-in to behavioral breakage with `1.0.x`; a future major release
will remove this feature and make its functionality the default.
- Implemented more traits for `Properties`.
- Changed `LineReader` to allow comments after section headers (#6).
- Slightly optimized glob performance.

Thanks to @kyle-rader-msft for contributing parser improvements!

## 1.0.2 (2023-03-23)

- Updated the test suite to demonstrate compliance with EditorConfig `0.15.1`.
- Fixed inconsistent character class behavior when
the character class does not end with `]`.
- Fixed redundant UTF-8 validity checks when globbing.
- Reorganized parts of the `glob` module to greatly improve code quality.

## 1.0.1 (2022-06-24)

- Reduced the MSRV for `ec4rs` to `1.56`, from `1.59`.

## 1.0.0 (2022-06-11)

Initial stable release!
