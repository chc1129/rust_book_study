fn main() {
    let a = ['a', 'b', 'c'];
    let iter = a.iter().enumerate();
    for (i, value) in iter {
        println!("({}, {})", i, value);
    }
}
