# fix-getters-calls

This package is a tool to fix getters call sites by removing the `get_` prefix
according to rules defined in [`rules`](../rules/README.md).

See the [workspace documentation](../README.md) for more details on `fix-getters`.

## Install

You can install `fix-getters-calls` using `cargo`, which allows invoking it like a
regular command.

```
cd fix-getters/fix-calls
cargo install --path ./
```

## Usage

**Warning:** by default, `fix-getters-calls` will overwrite the existing files.
See below if you want to check the output in a separate directory.

```
fix-getters-calls _PROJECT_PATH_
cd _PROJECT_PATH_
cargo fmt
```

Note that the getters definition won't be changed. Use [fix-def](../fix-def/README.md)
for that.

To test the result first, you can run:

```
fix-getters-calls _PROJECT_PATH_ _OUTPUT_PATH_
```

The project files won't be changed: modified files will be generated under
`_OUTPUT_PATH_`. Note however that only the modified files are generated, so
you won't be able to run `cargo fmt`.

## Uninstall

By default, `cargo` installs `fix-getters-calls` in its `bin` directory.
To uninstall, launch the following command:

```
cargo uninstall fix-getters-calls
```

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
