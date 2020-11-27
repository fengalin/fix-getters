/// This is a test

const MY_CONST_INSTANCE: MyType = MyType { foo: 42u64 };

const MY_CONST: u64 = MY_CONST_INSTANCE.get_foo();

const MY_CONST_NO_SELF: u64 = get_no_self(42u64);

static My_STATIC: u64 = MyType::get_no_self(42u64);

macro_rules! get_via_macro (
    ($self: expr) => ({
        let ret = $self.get_foo();
        ret
    })
);

const fn get_no_self(other: u64) -> u64 {
    other
}

fn from_my_type() -> u64 {
    let my_instance = MyType { foo: 42u64 };
    let other = my_instance.get_foo();
    let other = MyType { foo: other }.get_foo();
    let other = MyType { foo: other }.get_foo_param::<u64>();
    MyType::get_no_self(other)
}

fn from_my_type_might_be_bool() -> bool {
    let my_instance = MyType { foo: 42u64 };
    let (_, _) = (my_instance.get_mute(), my_instance.get_emit_eos());
    println!("{} {}", my_instance.get_mute(), my_instance.get_emit_eos());
    let _ = my_instance.get_do_ts_param::<u64>();
    // This one will fail unless we introduce a list of obvious booleans.
    my_instance.get_active()
}

struct MyType {
    foo: u64,
}

impl MyType {
    const fn get_no_self(other: u64) -> u64 {
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

    fn get_do_ts_param<T: From<bool>>(&self) -> T {
        true.into()
    }

    fn get_active(&self) -> bool {
        true
    }
}
