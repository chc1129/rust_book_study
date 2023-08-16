fn main() {
    let a = 10;                 // im,utable ojbect
    let a_ref = &a;             // reference
    let a_ref_copy = a_ref;     // copy reference
    print!("{} {} {}", a, a_ref, a_ref_copy); // borrow check!! - OK
}
