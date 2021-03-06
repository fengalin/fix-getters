# fix-getters-rules

This package contains rules definitions for the `fix-getters` tools.

See the [workspace documentation](https://github.com/fengalin/fix-getters/blob/0.3.2/README.md)
for more details on `fix-getters`.

## Rules

The `rules` apply to:

* file system directory entries to decide if a file should be processed or
  if a directory branch should be skipped. This requires `feature` **`dir-entry`**
  (enabled by default).
* functions name.

## Feature

* **`dir-entry`** — directory entry filtering rules. This features is enabled by
  default. Use `default-features = false` if your use case differs.

## Function name rules

The initial intent is to comply with Rust [naming conventions for getter methods](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html#getter/setter-methods-%5Brfc-344%5D):

> A method `foo(&self) -> &T` for getting the current value of the field.

### General rules

These rules are based on the function name and knowledge of the return type.
See next chapter for `get` functions returning exactly one `bool`.

A `get` function is considered eligible for `get` prefix removal if:

* The function starts with `get_`.
* The suffix is not a Rust keyword, which would result in invalid code.

  E.g.: `get_as`, `get_false`, ... are kept as is.

* The method would result inconsistent with other similar methods.

  E.g.: a `struct` `Value` with a `get_mut` method to get the underlying value
  as a mutable reference, `get_optional` to get the underlying value of a type
  in the form `Option<T>` and `get_some` to get the underlying value of a type
  for which the value is always defined.

  See `RESERVED` in [`function.rs`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/src/function.rs).

* The suffix is not part of a subsitution list. This includes some Rust keywords.

  E.g.: `get_type` is replaced with `type_`.

  See `EXACT_SUFFIX_SUBSTITUTES` in [`function.rs`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/src/function.rs).

Another rule is applied to homogenize functions names in the form
`get_something_mut`. This rule renames both `get_something_mut` and
`get_mut_something` as `something_mut`.

The `fix-getters` tools also apply selective rules based on on the function
signature. See the dedicated chapter in [this `README`](https://github.com/fengalin/fix-getters/blob/0.3.2/README.md#get-functions-selection).

### Functions returning exactly one `bool`

Get functions returning `bool` should usually use the form `is_suffix`, which
reads natural when used in a condition. E.g.:

```rust
if event.is_serialized() {
  ...
}
```

The following additional rules are implemented.

#### First token substitutions

When the suffix starts with a verb, it's common to conjugate. E.g.

* `get_emit_eos` -> `if element.emits_eos()`.

`BOOL_FIRST_TOKEN_SUBSTITUTES` in [`function.rs`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/src/function.rs)
lists a set of verbs and the matching substitutions and also includes other
cases such as:

* `get_always_...` -> `must_always_...`.
* `get_focus` -> `gets_focus`.

#### Special first tokens

Modal verbs should be kept unchanged and no `is` prefix should be used. E.g.:

* `get_can_focus` -> `can_focus`.
* `get_must_...` -> `must_...`.

This is also the case for already conjugated verbs. E.g.:

* `get_has_...` -> `has_...`.

See `BOOL_FIRST_TOKEN_NO_PREFIX` in [`function.rs`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/src/function.rs).

#### Exact suffix substitutions

In some cases, the semantic requires substitutions only if the whole suffix
matches a value. E.g.:

* `get_result` -> `result`. Neither `if a.is_result()` nor `if a.results()`
  would be suitable.
* `get_visibility` -> `is_visible` Neither `if a.is_visibility()` nor
  `if a.visibility()`) would be suitable.

See `BOOL_EXACT_SUBSTITUTES` in [`function.rs`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/src/function.rs).

#### get_is prefix

Finally, the `is` prefix shouldn't be repeated when already present:

* `get_is_active` -> `is_active`.

### Detecting functions returning exactly one `bool`

The return type of Rust functions is usually not explicit. When renaming the
`get` functions call sites (see [`fix-getters-calls`](https://github.com/fengalin/fix-getters/tree/0.3.2/fix-calls)),
the returned type must be inferred. The rules described in previous chapter are
reversed when possible and an additional heuristic is used: when the first token
of the `get` function suffix ends in `able`, the function is considered to be
returning a `bool`. E.g.:

* `get_seekable` -> `is_seekable`.

## LICENSE

This crate is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
