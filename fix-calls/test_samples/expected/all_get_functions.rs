//! This is a test
//!
//! ```rust
//! let b = String::from("abc");
//! assert_eq!(hello.str(), Some("Hello!"));
//! ```

/// ```
/// fn from_my_type() -> u64 {
///     let my_instance = MyType { foo: 42u64 };
///     let _ = my_instance.multiple_arg(42u64);
///     let other = my_instance.foo();
///     let _ = my_instance.type_();
/// }
/// ```
///
/// ```
/// macro_rules! via_macro (
///     ($self: expr) => ({
///         let _ = $self.result();
///         let _ = $self.multiple_arg(42u64);
///         let ret = $self.foo();
///         let _ = $self.type_();
///         let _ = MyType::type_();
///         ret
///     })
/// );
/// ```

const MY_CONST_INSTANCE: MyType = MyType { foo: 42u64 };

const MY_CONST: u64 = MY_CONST_INSTANCE.foo();

const MY_CONST_NOT_METHOD: u64 = not_method(42u64);

const MY_CONST_NOT_METHOD_PARAM: u64 = not_method_param::<u64>(42u64);

static My_STATIC: u64 = MyType::no_self(42u64);

const MY_TYPE: u64 = type_();

const MY_BOOLABLE: bool = is_boolable(42u64);

const MY_BOOL_IS_EQUAL: bool = is_equal(42u64);

const MY_BOOL_PARAM: bool = bool_param::<u64>(42u64);

macro_rules! get_via_macro (
    ($self: expr) => ({
        let _ = $self.does_ts_param::<u64>();
        let _ = $self.is_activable();
        let _ = $self.result();
        let _ = $self.multiple_arg($self.foo());
        let ret = $self.foo();
        let _ = $self.type_();
        let _ = MyType::type_();
        ret
    })
);

fn from_my_type() -> u64 {
    let my_instance = MyType { foo: 42u64 };
    let _ = my_instance.multiple_arg(my_instance.foo());
    let other = my_instance.foo();
    let other = MyType { foo: other }.foo();
    let _ = MyType { foo: other }.type_();
    let other = MyType { foo: other }.foo_param::<u64>();
    MyType::no_self(other)
}

fn from_my_type_might_be_bool() -> bool {
    let my_instance = MyType { foo: 42u64 };
    let (_, _) = (my_instance.is_muted(), my_instance.emits_eos());
    println!("{} {}", my_instance.is_muted(), my_instance.emits_eos());
    let _ = my_instance.emits_eos();
    let _ = my_instance.does_ts_param::<u64>();
    let _ = my_instance.does_ts();
    let _ = my_instance.is_active();
    let _ = my_instance.is_activable();
    let _ = my_instance.is_activable_bool();
    let _ = my_instance.result();
    // This one will fail unless we introduce a list of obvious booleans.
    my_instance.active()
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
        Self::no_self(self.foo)
    }

    fn other_foo(&self) -> u64 {
        self.foo()
    }

    fn macro_foo(&self) -> u64 {
        get_via_macro!(self)
    }

    fn get_foo_param<T: From<u64>>(&self) -> T {
        self.foo().into()
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
