macro_rules! S {
    ($t:tt) => { String::from($t) };
}

fn main() {
    println!("{}", S!("Hello, World!"));
}
