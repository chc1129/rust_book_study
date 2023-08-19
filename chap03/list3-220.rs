macro_rules! S {
    ($l:literal) => { String::from($l) };
}

fn main() {
    println!("{}", S!("Hello, World!"));
}
