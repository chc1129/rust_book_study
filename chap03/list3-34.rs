fn main() {
    let mut a = 10;             // mutable object
    let a_mut_ref = &mut a;     // mutable reference
    *a_mut_ref = 20;            // dereference and assign
    println!("{}", a_mut_ref);  // auto dereference
}
