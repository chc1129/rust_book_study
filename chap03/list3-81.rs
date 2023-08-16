fn main() {
    let mut x = 0;
    'a: while {
        x += 1;
        if x > 5 { break 'a; }
        x < 10
    } {}
    dbg!(x); // x=6
}
