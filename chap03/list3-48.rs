fn main() {
    let mut s = "Hello, World!".to_string();
    let first = &mut s[..5];
    let last = &mut s[5..];
    first.make_ascii_uppercase();
    last.make_ascii_lowercase();
    println!("{}", s);
}
