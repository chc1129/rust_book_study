fn main() {
    let mut x = 0;
    'outer: loop {
        'inner: while {
            x += 1;
            if x > 5 { break 'outer; }
            x < 10
        } {}
    }
    dbg!(x); // x=6
}
