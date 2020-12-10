//! This is a test
//!
//! ```rust
//! let b = String::from("abc");
//! assert_eq!(hello.get_str(), Some("Hello!"));
//! ```

/// ```
/// fn from_my_type() -> u64 {
///     let my_instance = MyType { foo: 42u64 };
///     let _ = my_instance.get_multiple_arg(42u64);
///     let other = my_instance.get_foo();
/// }
/// ```
///
/// ```
/// macro_rules! get_via_macro (
///     ($self: expr) => ({
///         let _ = $self.get_result();
///         let _ = $self.get_multiple_arg(42u64);
///         let ret = $self.get_foo();
///         ret
///     })
/// );
/// ```

const MY_CONST_INSTANCE: MyType = MyType { foo: 42u64 };

const MY_CONST: u64 = MY_CONST_INSTANCE.get_foo();

const MY_CONST_NOT_METHOD: u64 = get_not_method(42u64);

const MY_CONST_NOT_METHOD_PARAM: u64 = get_not_method_param::<u64>(42u64);

static My_STATIC: u64 = MyType::get_no_self(42u64);

const MY_BOOLABLE: bool = get_boolable(42u64);

const MY_BOOL_IS_EQUAL: bool = get_is_equal(42u64);

const MY_BOOL_PARAM: bool = get_bool_param::<u64>(42u64);

macro_rules! get_via_macro (
    ($self: expr) => ({
        let _ = $self.get_do_ts_param::<u64>();
        let _ = $self.get_activable();
        let _ = $self.get_result();
        let _ = $self.get_multiple_arg($self.get_foo());
        let ret = $self.get_foo();
        ret
    })
);

fn from_my_type() -> u64 {
    let my_instance = MyType { foo: 42u64 };
    let _ = my_instance.get_multiple_arg(my_instance.get_foo());
    let other = my_instance.get_foo();
    let other = MyType { foo: other }.get_foo();
    let other = MyType { foo: other }.get_foo_param::<u64>();
    MyType::get_no_self(other)
}

fn from_my_type_might_be_bool() -> bool {
    let my_instance = MyType { foo: 42u64 };
    let (_, _) = (my_instance.get_mute(), my_instance.get_emit_eos());
    println!("{} {}", my_instance.get_mute(), my_instance.get_emit_eos());
    let _ = my_instance.get_is_emit_eos();
    let _ = my_instance.get_do_ts_param::<u64>();
    let _ = my_instance.get_does_ts();
    let _ = my_instance.get_is_active();
    let _ = my_instance.get_activable();
    let _ = my_instance.get_activable_bool();
    let _ = my_instance.get_result();
    // This one will fail unless we introduce a list of obvious booleans.
    my_instance.get_active()
}

const fn get_not_method(other: u64) -> u64 {
    other
}

const fn get_not_method_param<T: Sized>(other: T) -> T {
    other
}

const fn get_boolable(other: u64) -> bool {
    other == 42u64
}

const fn get_is_equal(other: u64) -> bool {
    other == 42u64
}

const fn get_bool_param<T: Sized>(other: T) -> bool {
    true
}

// From here on, these are type and method definition
// so that the code above gets validated by rls / analyzer.
struct MyType {
    foo: u64,
}

impl MyType {
    const fn get_no_self(other: u64) -> u64 {
        other
    }

    fn get_multiple_arg(&self, other: u64) -> u64 {
        other
    }

    const fn get_foo(&self) -> u64 {
        Self::get_no_self(self.foo)
    }

    fn other_foo(&self) -> u64 {
        self.get_foo()
    }

    fn macro_foo(&self) -> u64 {
        get_via_macro!(self)
    }

    fn get_foo_param<T: From<u64>>(&self) -> T {
        self.get_foo().into()
    }

    fn get_mute(&self) -> bool {
        true
    }

    fn get_emit_eos(&self) -> bool {
        true
    }

    fn get_is_emit_eos(&self) -> bool {
        true
    }

    fn get_does_ts(&self) -> bool {
        true
    }

    fn get_do_ts_param<T: From<bool>>(&self) -> T {
        true.into()
    }

    fn get_active(&self) -> bool {
        true
    }

    fn get_is_active(&self) -> bool {
        true
    }

    fn get_activable(&self) -> bool {
        true
    }

    fn get_activable_bool(&self) -> bool {
        true
    }

    fn get_result(&self) -> bool {
        true
    }
}
