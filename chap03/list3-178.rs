fn main() {
    // for
    for i in 0..10 {
        print!("{},", i);
    }
    println!("");

    // for-each
    (0..10).for_each(|i| print!("{},", i));
}
