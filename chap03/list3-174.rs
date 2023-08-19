fn main() {
    let a = [1, 2, 3];
    let iter = a.iter().map(|x| 2 * x);
    for i in iter {
        println!("{:?}", i);
    }
}
