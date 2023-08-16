fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x >= 10 {
            break;
        }
    }
    dbg!(x); // x=10
}
