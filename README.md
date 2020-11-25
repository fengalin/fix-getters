# fix-getters

This repository contains crates to help with the removal of the `get_` prefix
in existing getters.

Attempts at removing those automatically or manually proved to be suboptimal.
E.g.:

- Some `get_*` functions actually retrieve data, so it's not always a good idea
  to remove the `get` semantic.
- After removing the `get_` prefix, it is a good idea to add a
  `[doc(alias = "get_name")]` so that users can retrive the new name in the
  crate's documentation. This is particulary helpful for bindings since users
  might be experimented with C code or refer to examples with C code.
  However, the `[doc..]` attribute is only useful in global functions,
  `struct` `impl` or `trait` definition. It's unneeded in `trait` implementations
  for `struct`.
- Then it's necessary to update the call sites.

## Packages

This workspace contains the following packages:

- [utils](utils/README.md): functions which are common to the executables.
- [rules](rules/README.md): rules which are applied during the update process.
- [fix-definition](fix-definition/README.md): executable which updates the
  getters defintion.

## LICENSE

All crates contained in here are licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
