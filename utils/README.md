# fix-getters-utils

This package contains functions which are common to the `fix-getters` tools.

See the [workspace documentation](../README.md) for more details on `fix-getters`.

## Utils

The `utils` functions provide features such as:

- `type`s and `trait`s to help building collectors of renamable functions.
- a crate traversal mechanism which complies with the directory entry rules
  defined in [rules](../rules/).
- a common `Error` which can be handled in `main`.
- a Rust scope tracker which helps figure out the context of a function.

## Features

The optional feature is enabled by default. Use `default-features = false` if
your use case differs.

- **`log`** — Logging via the `log` crate.

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
