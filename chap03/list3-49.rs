fn main() {
    let mut s = "Hello, World!".to_string();
    let (first, last) = s.split_at_mut(5);
    first.make_ascii_uppercase();
    last.make_ascii_lowercase();
    println!("{}", s);
}
