fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let iter = a1.iter().zip(a2.iter());
    for i in iter {
        println!("{:?}", i);
    }
}
