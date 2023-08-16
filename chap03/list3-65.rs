use std::fmt;

struct Hoge;

impl fmt::Display for Hoge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hoge is unit-like sruct")
    }
}

fn main() {
    let hoge = Hoge{};
    println!("{}", hoge);
}
