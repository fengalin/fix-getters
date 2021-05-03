# fix-getters-def

This package is a tool to fix getters definitions by removing the `get` prefix
according to rules defined in [`rules`](../rules/). It can also optionally
insert a `[doc(alias = "get_name")]` attribute where necessary so that the new
name for the getter can be retrieved from the documentations by searching
previous name.

See the [workspace documentation](../README.md) for more details about
`fix-getters`.

## Install

You can install `fix-getters-def` using `cargo`, which allows invoking the tool
as a regular command.

### From crates.io

```
cargo install fix-getters-def
```

### From source

```
cargo install --path fix-def
```

## Usage

### Default invocation

**Warning:** by default, `fix-getters-def` will overwrite existing files.
See below if you want to check the output in a separate directory.

This will fix the project in current directory:

```
fix-getters-def
cargo fmt
```

Note that the call sites won't be changed. Use [fix-calls](../fix-calls/) for
that.

To test the result first, you can run:

```
fix-getters-def _PROJECT_PATH_ _OUTPUT_PATH_
```

The project files won't be changed: modified files will be generated under
`_OUTPUT_PATH_`. Note however that only the modified files are generated, so
you won't be able to run `cargo fmt`.

### Conservative get function identification 

Use the `--conservative` option (short `-c`) if you prefer applying a
conservative approach based on the `get` function signature. By default, all
`get` functions are renamed.

See the [workspace documentation](../README.md#get-functions-selection) for more
details about the conservative identification mode.

### doc alias attributes

By default, `fix-getters-def` adds a doc alias attribute with the original name
for the renamed functions.

Use the `--no-doc-aliases` option (short `-n`) if you don't want to generate the
doc alias attributes.

Prior to version `0.3.1`, doc aliases were added only if the `--doc-alias`
option (short `-d`) was provided. This option is now deprecated and will be
removed in the next major version.

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
