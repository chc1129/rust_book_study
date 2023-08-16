fn main() {
    let a: Option<String> = Some(String::from("hello"));
    let a = match a {
        Some(x) => { println!("{}", x); Some(x) }
        None => None,
    };
    println!("{:?}", a); // borrow check!! - OK
}
