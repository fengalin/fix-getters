//! In doc:
//!
//! ```rust
//! struct MyType(u64);
//! impl MyType {
//!     pub fn get_foo(&self) -> u64 {
//!         self.0
//!     }
//! }
//! ```
//!
//! ```
//! macro_rules! get_from_macro(
//!     ($name:ident) => {
//!         impl $name {
//!             fn get_from_macro(&self) -> u64 {
//!                 self.0
//!             }
//!         }
//!     }
//! );
//! ```

/// This is a test
#[doc(alias = "get_foo")]
pub fn foo() -> u64 {
    42u64
}

pub const fn get_foo() -> u64 {
    42u64
}

fn get_with_type_param<T: From<u64>>() -> T {
    42u64.into()
}

fn get_bool_type_param<T: Into<u64>>(other: T) -> bool {
    42u64 == other.into()
}

struct MyType {
    foo: u64,
}

impl MyType {
    #[doc(alias = "get_property_foo")]
    pub fn get_foo(&self) -> u64 {
        self.foo
    }

    fn get_mut(&mut self) -> &mut Self {
        self
    }

    fn get_mut_structure(&mut self) -> &mut Self {
        self
    }

    fn get_struct_mut(&mut self) -> &mut Self {
        self
    }

    async fn get_foo_async(&self) -> u64 {
        self.foo
    }

    const fn get_foo_const(&self) -> u64 {
        self.foo
    }

    const fn get_boolean(&self) -> bool {
        true
    }

    const fn get_mute(&self) -> bool {
        true
    }

    const fn get_emit_eos(&self) -> bool {
        true
    }

    const fn get_is_emit_eos(&self) -> bool {
        true
    }

    const fn get_is_activated(&self) -> bool {
        true
    }

    fn get_has_entry(&self) -> bool {
        true
    }

    fn get_does_ts(&self) -> bool {
        true
    }

    fn get_not_self(other: u64) -> u64 {
        other
    }

    fn get_foo_with_arg(&self, _other: u64) -> u64 {
        self.foo
    }

    fn get_foo_with_lt<'a>(&'a self) -> &'a u64 {
        &self.foo
    }

    fn get_bool_with_param<T: Into<u64>>(&self, other: T) -> bool {
        self.foo == other.into()
    }

    fn get_foo_with_param<T: From<u64>>(&self) -> T {
        self.foo.into()
    }

    fn not_get(&self) -> u64 {
        self.foo
    }
}

macro_rules! get_from_macro(
    ($name:ident, $type_:ty) => {
        impl $name {
            fn get_from_macro(&self) -> u64 {
                self.foo
            }

            fn get_42(&self) -> bool {
                self.foo == 42u64
            }

            fn get_complexe(&self) -> (u64, bool) {
                (self.foo, self.foo == 42u64)
            }

            fn get_multiple_arg(&self, other: u64) -> bool {
                self.foo == other
            }

            fn get_non_self_unique_arg(other: u64) -> u64 {
                other
            }

            fn get_foo_with_lt_from_macro<'a>(&'a self) -> &'a u64 {
                &self.foo
            }

            fn get_boolable_with_param_from_macro<T: Into<u64>>(&self, other: T) -> bool {
                self.foo == other.into()
            }

            fn get_not_obvious_bool_with_param_from_macro<T: Into<u64>, $type_>(&self, other: T) -> bool {
                self.foo == other.into()
            }

            fn get_param_from_macro<T: From<u64>, $type_>(&self) -> (T, bool) {
                (self.foo.into(), self.foo == 42u64)
            }

            fn not_get_from_macro(&self) -> bool {
                self.foo == 42u64
            }
        }

        fn get_sandalone(arg: u64) -> u64 {
            arg
        }

        fn get_bool_sandalone(arg: u64) -> bool {
            arg == 42u64
        }
    }
);

get_from_macro!(MyType);

trait MyTrait {
    fn get_trait_no_impl(&self) -> u64;

    fn get_trait_impl(&self) -> u64 {
        self.get_trait_no_impl()
    }
    fn get_trait_impl_param<T: From<u64>>(&self) -> T;
}

impl MyTrait for MyType {
    fn get_trait_no_impl(&self) -> u64 {
        42u64
    }

    fn get_trait_impl_param<T: From<u64>>(&self) -> T {
        self.get_trait_no_impl().into()
    }
}

impl<'a> MyTrait for &'a [MyType] {
    fn get_trait_no_impl(&self) -> u64 {
        42u64
    }

    fn get_trait_impl_param<T: From<u64>>(&self) -> T {
        self.get_trait_no_impl().into()
    }
}

trait Test: std::fmt::Debug {}

impl<'a> MyTrait for &'a [&'a (dyn Test + Send)] {
    fn get_trait_no_impl(&self) -> u64 {
        42u64
    }

    fn get_trait_impl_param<T: From<u64>>(&self) -> T {
        self.get_trait_no_impl().into()
    }
}

impl<'a> MyTrait for &'a (u64, bool) {
    fn get_trait_no_impl(&self) -> u64 {
        42u64
    }

    fn get_trait_impl_param<T: From<u64>>(&self) -> T {
        self.get_trait_no_impl().into()
    }
}
