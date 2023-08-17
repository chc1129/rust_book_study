use std::default::Default;

trait Trait {
    fn echo(&self) {}
}

#[derive(Default)]
struct Hoge;
impl Trait for Hoge {
    fn echo(&self) { println!("Hoge"); }
}

#[derive(Default)]
struct Foo;
impl Trait for Foo {
    fn echo(&self) { println!("Foo"); }
}

fn foo_generics<T: Trait + Default>() -> T { T::default() }
fn foo_impl_trait() -> impl Trait { Hoge::default() }

fn main() {
    foo_generics::<Hoge>().echo();
    foo_generics::<Foo>().echo();
    foo_impl_trait().echo();
}
