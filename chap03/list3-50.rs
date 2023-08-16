fn main() {
    let a = 10;             // immutable object
    let b = a;              // copy
    print!("{} {}", a, b);  // borrow check!! - OK
}
