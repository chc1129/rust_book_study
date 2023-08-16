fn main() {
    let a = 10;         // immutable object
    let aref1 = &a;     // reference
    let aref2 = &a;     // reference
    println!("{}, {}, {}", a, aref1, aref2); // borrow check!! - OK
}
