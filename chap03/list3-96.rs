fn main() {
    let a: Option<String> = Some(String::from("hello"));
    match a {
        Some(x) => println!("{}"< x),   // move ownership
        None => ()
    }
    println!("{:?}", a);
    //               ^ value borrowed here after partial move
    // error[E0382]: borrow of partiall moved valu: `a`
}
