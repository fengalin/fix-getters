# fix-getters-rules

This package contains rules definition to be used in `fix-getters` tools.

See the [workspace documentation](../README.md) for more details on `fix-getters`.

## Rules

The `rules` apply to:

- file system directory entries to decide if a file should be processed or
  if a directory branch should be skipped. This requires `feature` **`dir-entry`**
  (enabled by default).
- functions name and signature.

## Features

- **`fs`** — File system traversal helper. This features is enabled by default.
  Use `default-features = false` if your use case differs.

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
