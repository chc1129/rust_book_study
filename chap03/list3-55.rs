fn main() {
    let a = 42;
    let ref_ref_ref_a = &&&a;
    let ref_a = **ref_ref_ref_a;
    let b = *ref_a;
    print!("{} {}", a, b);
}
