fn main() {
    println!("Hello {:<5}!", "x");      // Hello x    !
    println!("Hello {:-<5}!", "x");     // Hello x----!
    println!("Hello {:^5}!", "x");      // Hello   x  !
    println!("Hello {:>5}!", "x");      // Hello     x!
}
