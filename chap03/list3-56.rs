fn main() {
    let a           = 10;       // immutable object
    let a_ref       = &a;       // reference
    let a_ref_ref   = &a_ref;   // reference to reference
    println!("{}", a == a_ref);
    // error[E0277]: can't compare `{interger}` with `&{integer}`
    println!("{}", a_ref_ref == a_ref);
     // error[E0277]: can't compare `{interger}` with `&{integer}`
}
