fn main() {
    let a = [0i32, 1, 2];
    let iter = a.iter().filter(|x| x.is_positive());
    for i in iter {
        println!("{:?}", i);
    }
}
