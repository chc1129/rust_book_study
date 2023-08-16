fn main() {
    let mut a = 10;           // mutable object
    let a_ref1 = &a;          // reference
    let a_mut_ref1 = &mut a;  // mutable reference
    let a_mut_ref2 = &mut a;  // この時点でa_ref1, a_mut_ref1は存在しない
    *a_mut_ref1 = 20;         // assign(error)
    println!("{}", a);        // borrow check!! - Erro!
}
