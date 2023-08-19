#[drive(Default, Debug)]
enum Kind {
    Hoge,
    #[default]
    Foo,
    Bar,
}

fn main() {
    let value = Kind::default();
    println!("{:?}", value);
}
