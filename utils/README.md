# fix-getters-utils

This package contains functions which are common to the `fix-getters` tools.

See the [workspace documentation](../README.md) for more details on `fix-getters`.

## Utils

The `utils` functions provide features such as:

- a common `Error` which can be handled in `main`.
- a file system traversal function which complies with the [rules](../rules/).
- a Rust scope tracker which helps figure out the context where a function is
  defined or used.

## Features

All the features are enabled by default. Use `default-features = false` and add
specific features if your use case differs.

- **`fs`** — File system traversal helper.
- **`log`** — Logging via the `log` crate.
- **`parser`** — Rust code parser related utilities.
  This features enables both **`parser-error`** & **`scope`**.
- **`parser-error`** — Rust code parser error wrapper.
- **`scope`** — Rust code `Scope` identification.

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
