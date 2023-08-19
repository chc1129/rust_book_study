struct Hoge {
    foo: i32,
    bar: i32,
}

impl Default for Hoge {
    fn default() -> Self { Self { foo: 2, bar: -5 } }
}

fn main() {
    let hoge = Hoge::default();
    println!("foo: {}, bar: {}", hoge.foo, hoge.bar);
}
