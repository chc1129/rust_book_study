#[derive(Default)]
struct Hoge {
    foo: i32,
    bar: i32,
}

fn main() {
    let hoge = Hoge::default();
    println!("foo: {}, bar: {}", hoge.foo, hoge.bar);
}
