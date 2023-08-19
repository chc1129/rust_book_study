macro_rules! S {
    ($e:expr) => { String::from($e) };
}

fn main() {
    println!("{}", S!("Hello, World!"));
}
