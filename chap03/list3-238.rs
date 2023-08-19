#[derive(Default)]
struct Hoge {
    foo: i32,
    bar: i32,
}

fn main() {
    let hoge = Hoge { foo: 10, ..Default::default() };
    println!("foo: {}, bar: {}", hoge.foo, hoge.bar);
}
