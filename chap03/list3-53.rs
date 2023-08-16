fn main() {
    let mut a = 10;                 // mutable object
    let a_mut_ref = &mut a;         // mutable reference
    let a_mut_ref_move = a_mut_ref; // move mutable reference
    print!("{}", a_mut_ref_move);   // borrow check!! - OK
}
