fn main() {
    let mut a = 18;           // mutable object
    let a_ref1 = &a;          // reference
    let a_mut_ref1 = &mut a;  // mutable reference
    let a_mut_ref2 = &mut a;  // mutable reference
    *a_mut_ref2 = 20;         // assign
    println!("{}", a);        // borrow check!! - OK
}
