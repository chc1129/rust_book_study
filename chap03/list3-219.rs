macro_rules! Foo {
    () => { println!("Hello, World!"); };
}

fn main() {
    Foo!();
    Foo![];
    Foo!{};
}
