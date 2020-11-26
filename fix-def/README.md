# fix-getters-definition

This package is an utility to fix getters definitions by removing the `get_`
prefix according to rules defined in [`rules`](../rules/README.md). It also
inserts a `[doc(alias = "get_name")]` attribute so that the new name for the
getter can be retrieved from the documentations by searching for previous name.

See the [workspace documentation](../README.md) for more details on `fix-getters`.

## Install

You can install `fix-getters-definition` using `cargo`, which allows invoking it
like a regular command.

```
cd fix-getters/fix-definition
cargo install --path ./
```

## Usage

**Warning:** by default, `fix-getters-definition` will overwrite the existing
files. See below if you want to check the output in a separate directory.

```
fix-getters-definition _PROJECT_PATH_
cd _PROJECT_PATH_
cargo fmt
```

Note that the call sites won't be changed. Use [fix-call-sites](../fix-call-sites/README.md)
for that.

To test the result first, you can run:

```
fix-getters-definition _PROJECT_PATH_ _OUTPUT_PATH_
```

The project files won't be changed: modified files will be generated under
`_OUTPUT_PATH_`. Note however that only the modified files are genereted, so
you won't be able to run `cargo fmt`.

## Uninstall

By default, `cargo` installs the `fix-getters-definition` executable in
its `bin` directory. You can get it with:

```
which cargo
```

To uninstall `fix-getters-definition`, just remove it from that directory.

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
