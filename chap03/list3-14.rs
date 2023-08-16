fn main() {
    println!("Hello {:5}!", "x");       // Hello x  !
    println!("Hello {:1$}!", "x", 5);   // Hello x  !
    println!("Hello {1:0$}!", 5, "x");  // Hello x  !
}
