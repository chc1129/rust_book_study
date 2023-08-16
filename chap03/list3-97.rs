fn main() {
    let a: Option<String> = Some(String::from("hello"));
    match a {
        Some(ref x) => println!("{}", x), // reference
        None => ()
    }
    println!("{:?}", a); // borrwo check!! - OK
}
