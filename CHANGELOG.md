# Changelog

Releases following pre-releases shall always be equivalent to the pre-release
except for the version number and possibly dependency versions. As such,
changes listed for each release shall be relative to the previous release,
excluding any pre-releases in between.

## ec4rs 2.0.0-rc.1 (2026-07-21)

- Reworked glob support.
  - The usual glob implementation is now in the `ec4rs_glob` crate.
  - Added the `Pattern` trait for glob patterns from other glob engines.
  - Added opt-in support for `globset` as an alternate glob engine.
  - Many types and functions are now generic over `<P: Pattern>`.
- Reworked `RawValue` into `SharedString`.
  - `SharedString` accepts the empty and `"unset"` values,
    is internally reference-counted, and uses `'static` values where possible.
  - Added the `Cache` trait for string-level caching.
  - Added the `ToSharedString` trait for efficient string conversions.
  - Slimmed down `PropertyValue` by moving functionality to supertraits.
- Reworked the property enums.
  - Added `Unset` as a default value for all the enums.
  - All property enums are now `non_exhaustive`.
  - `MaxLineLen` now has `Unset` instead of `Off` (#19).
- Reworked `SpellingLanguage`.
  - Now available without a feature flag, but only accepts EditorConfig.
  - With the `bcp_47` feature, accepts any BCP 47 locale tag.
  - Fixed `unset` being parsed as an actual value.
- Reworked line parsing and its errors.
  - Added `ParseError::InvalidSection` to handle cases where a line is
    probably a section but is not a valid one.
  - Trailing comments on section headers, which `ec4rs` supports as an
    extension to the spec, are handled more-reliably.
- Added `Source` as a standard way of working with line location data.
  - Most functions that took path + line number pairs now take `Source`.
  - Merged `Error::Parse` and `Error::InFile` by making the former contain
    an `Option<Source>`.
- Added the `PropertiesSink` trait.
  - `PropertiesSource::apply_to` now takes a
    `&mut (impl PropertiesSink + ?Sized)` instead of `&mut Properties`.
  - `PropertiesSink` is implemented for `Properties`.
- Added a stub `Preamble` type.
  - Currently only contains the value of the `root` key-value pair,
    defaulting to `false` if unspecified. May gain fields in the future if
    the spec calls for them.
  - Replaces the `is_root` field on `ConfigParser`.
- Changed functions that took `impl AsRef` to take `&(impl AsRef + ?Sized)`.
- Removed the `allow-empty-values` and `language-tags` features.
- `ConfigFiles` now uses `std::path::absolute` instead of custom logic for
  resolving relative paths.
- Fixed leading `U+FEFF` not being stripped from all files (#10).
  It is now stripped from the start of each line.
- Fixed missing negation in `LineReader::has_more` (#21).

### Dependency changes

- Increased MSRV to 1.79.
- Added optional dependency on `ec4rs_glob` 1.0.0-rc.1.

## ec4rs_glob 0.1.0 (2026-07-21)

Initial release!

## ec4rs 1.2.0 (2025-04-19)

- Added feature `track-source` to track where any given value came from.
- Added `-0Hl` flags to `ec4rs-parse` for displaying value sources.
- Added `RawValue::to_lowercase`.
- Implemented `Display` for `RawValue`.
- Changed `ec4rs-parse` to support empty values for compliance with
  EditorConfig `0.17.2`.
- Fixed fallbacks adding an empty value for `indent_size`.
- Fixed `Properties::iter` and `Properties::iter_mut` not returning
pairs with empty values when `allow-empty-values` is enabled.

## ec4rs 1.1.1 (2024-08-29)

- Update testing instructions to work with the latest versions of cmake+ctest.
- Fix `/*` matching too broadly (#12).

## ec4rs 1.1.0 (2024-03-26)

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

## ec4rs 1.0.2 (2023-03-23)

- Updated the test suite to demonstrate compliance with EditorConfig `0.15.1`.
- Fixed inconsistent character class behavior when
  the character class does not end with `]`.
- Fixed redundant UTF-8 validity checks when globbing.
- Reorganized parts of the `glob` module to greatly improve code quality.

## ec4rs 1.0.1 (2022-06-24)

- Reduced the MSRV for `ec4rs` to `1.56`, from `1.59`.

## ec4rs 1.0.0 (2022-06-11)

Initial stable release!
