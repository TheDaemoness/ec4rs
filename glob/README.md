# ec4rs_glob

The globbing engine used by [`ec4rs`](https://github.com/TheDaemoness/ec4rs).
Please refer to that project for licensing and contribution info.

You're probably better-served by using the
[`glob` crate](https://crates.io/crates/glob),
or the [`globset` crate](https://crates.io/crates/globset),
both of which are widely-used and have good maintenance status at the time of
writing. The only reason to use this crate is if you need:

- Numeric range patterns (e.g. `{1..42}`),
- An incredibly permissive glob engine, or
- A glob engine that is more featureful than `glob`
  but lighter than `globset` (which pulls in parts of `regex`).

## Glob Features

This crate supports the exact set of features necessary for
perfect EditorConfig compliance, including passing the entire suite of
glob tests for EditorConfig cores. Details can be found
[on the EditorConfig specification](https://editorconfig.org/#wildcards).
