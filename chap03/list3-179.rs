fn main() {
    let a = [1, 4, 2, 3];
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("about to filter: {}", x))
        .fold(0, |sum, i| sum + i);
    println!("{}", sum);
}
