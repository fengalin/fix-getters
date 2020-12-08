# fix-getters-def

This package is a tool to fix getters definitions by removing the `get` prefix
according to rules defined in [`rules`](../rules/). It can also optionally
insert a `[doc(alias = "get_name")]` attribute where necessary so that the new
name for the getter can be retrieved from the documentations by searching
previous name.

See the [workspace documentation](../README.md) for more details on `fix-getters`.

## Install

You can install `fix-getters-def` using `cargo`, which allows invoking the tool
as a regular command.

```
cd fix-getters/fix-def
cargo install --path .
```

## Usage

**Warning:** by default, `fix-getters-def` will overwrite existing files.
See below if you want to check the output in a separate directory.

```
fix-getters-def _PROJECT_PATH_
cd _PROJECT_PATH_
cargo fmt
```

You can also omit the _PROJECT_PATH_ in which case current directory will be used.

Use the `--doc-alias` option (short `-d`) if you want to generate a doc alias
attribute with the original name for the renamed functions.

Note that the call sites won't be changed. Use [fix-calls](../fix-calls/) for
that.

To test the result first, you can run:

```
fix-getters-def _PROJECT_PATH_ _OUTPUT_PATH_
```

The project files won't be changed: modified files will be generated under
`_OUTPUT_PATH_`. Note however that only the modified files are generated, so
you won't be able to run `cargo fmt`.

## Uninstall

To uninstall, launch the following command:

```
cargo uninstall fix-getters-def
```

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
