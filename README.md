# fix-getters ![CI](https://github.com/fengalin/fix-getters/workflows/CI/badge.svg) [![dependency status](https://deps.rs/repo/github/fengalin/fix-getters/status.svg)](https://deps.rs/repo/github/fengalin/fix-getters)

This repository contains crates and tools to help with the removal of the `get`
prefix from getters in existing Rust code.

Rust [naming conventions for getter methods](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html#getter/setter-methods-%5Brfc-344%5D)
stipulates to use:

> A method `foo(&self) -> &T` for getting the current value of the field.

Attempts at removing the `get` prefix automatically or manually proved to be
suboptimal with large code bases:

* Some `get_*` functions actually perform some sort of computation, so we can't
  exactly consider they return the current value of a field.
* Removing the `get` prefix automatically can result in invalid code.
  Ex. `get_mut`, `get_loop`, ...
* Getters returning a `bool` should usually use the form `is_suffix`, but
  sometimes, we just want to conjugate a verb. E.g.: `if element.emits_eos()`.
* Most `get_*` functions that return a `bool` and which actually perform some sort
  of computation can still be renamed as `is_suffix`, since it makes sense in
  expressions such as `if context_a.is_same_as(&context_b)`.
* Then it's necessary to update the getters call sites and we have to guess
  the return type somewhat.

## Get functions selection

Not all `get` functions should be renamed. Besides the [`rules`](https://github.com/fengalin/fix-getters/blob/0.3.2/rules/README.md#function-name-rules)
enforced while renaming the function, other criteria are observed before
deciding whether to apply the new name. `get` functions returning exactly one
`bool` require special attention since they are usually renamed using an `is`
prefix or a conjugated verb which make the new name suitable in expressions.

By default, all `get` functions are renamed. However, users might prefer sticking
by the definition and only rename getters which actually return a field.

The following rules apply to `get` functions not returning exactly one `bool`.
They are enforced by the tools when the `conservative` mode is selected. Note
that this might leave behind candidates. Use this if you consider it easier to
manually change these candidates after the automatic pass than searching in the
automatic changes.

* The function is a method. This excludes standalone functions or associated
  functions.
* The function accepts no arguments besides `&['_][mut] self`. The methods which
  accept other arguments are not `getter`s in the sense that they usually don't
  return current value of a field. Similarly, functions consuming `self` are not
  eligible for `get` removal.
* The function accepts no type parameter (lifetimes are accepted). The reason is
  the same as for functions accepting multiple arguments (see above).

These rules are implemented in the [fix-def](https://github.com/fengalin/fix-getters/tree/0.3.2/fix-def)
& [fix-calls](https://github.com/fengalin/fix-getters/tree/0.3.2/fix-calls)
tools and apply to regular code, macros and documentation code.

## Packages

This workspace contains the following packages:

* [rules](https://github.com/fengalin/fix-getters/tree/0.3.2/rules):
  rules to apply during the update process.
* [fix-def](https://github.com/fengalin/fix-getters/tree/0.3.2/fix-def):
  a tool to update the `get` functions definition.
* [fix-calls](https://github.com/fengalin/fix-getters/tree/0.3.2/fix-calls):
  a tool to update the `get` functions call sites.
* [utils](https://github.com/fengalin/fix-getters/tree/0.3.2/utils):
  utilities and types common to the `fix-getters` tools.

## LICENSE

All crates contained in here are licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
