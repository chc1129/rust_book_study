fn main() {
    let mut a = 10;             // mutable object
    let a_ref1 = &a;            // reference
    let a_mut_ref1 = &mut a;    // mutable reference
    let a_mut_ref2 = &mut a;    // この時点でa_ref1, a_mut_ref1は存在しない
    let a_ref2 = &a;            // この時点でa_mut_ref2は存在しない
    // println!("{}". a_mut_ref2);       // borrow check!! - Error
    // println!("{} {}", a_ref1, aref2); // borrow check!! - Error
    println!("{}", a_ref2);     // borrow check!! - OK
}
