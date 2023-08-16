fn main() {
    let mut x = 0;
    while {
        x += 1;
        x < 10
    } {}
    dbg!(x); // x=10
}
