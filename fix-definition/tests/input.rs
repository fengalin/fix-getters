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

    fn get_not_self(other: u64) -> u64 {
        other
    }

    fn get_foo_with_arg(&self, _other: u64) -> u64 {
        self.foo
    }

    fn get_foo_with_param<T: From<u64>>(&self) -> T {
        self.foo.into()
    }
}

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
