# fix-getters-calls

This package is a tool to fix getters call sites by removing the `get_` prefix
according to [`rules`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/README.md#function-name-rules).

See the [workspace documentation](https://github.com/fengalin/fix-getters/blob/0.3.2/README.md)
for more details on `fix-getters`.

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

Note that the getters definition won't be changed. Use [fix-def](https://github.com/fengalin/fix-getters/tree/0.3.2/fix-def)
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

See the [workspace documentation](https://github.com/fengalin/fix-getters/blob/0.3.2/README.md#get-functions-selection)
for more details on the conservative identification mode.

## Uninstall

To uninstall, use:

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
