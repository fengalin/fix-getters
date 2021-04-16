# fix-getters-calls

This package is a tool to fix getters call sites by removing the `get_` prefix
according to rules defined in [`rules`](../rules/).

See the [workspace documentation](../README.md) for more details about
`fix-getters`.

## Install

You can install `fix-getters-calls` using `cargo`, which allows invoking the
tool as a regular command.

### From crates.io

```
cargo install fix-getters-calls
```

### From source

```
cargo install --path fix-calls
```

## Usage


### Default invocation

**Warning:** by default, `fix-getters-calls` will overwrite existing files.
See below if you want to check the output in a separate directory.

This will fix the project in current directory:

```
fix-getters-calls
cargo fmt
```

Note that the getters definition won't be changed. Use [fix-def](../fix-def/)
for that.

To test the result first, you can run:

```
fix-getters-calls _PROJECT_PATH_ _OUTPUT_PATH_
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

## Uninstall

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
