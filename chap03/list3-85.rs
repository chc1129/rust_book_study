fn main() {
    let s = String::from("hello");
    println!("{}", &s[0..2]);           // he
    println!("{}", &s[0..=2]);          // hel
    println!("{}", &s[..2]);            // he
    println!("{}", &s[3..s.len()]);     // lo
    println!("{}", &s[3..]);            // lo
    println!("{}", &s[..]);             // hello
}
