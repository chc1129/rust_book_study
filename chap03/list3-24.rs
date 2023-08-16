fn main() {
    let a = 18;

    { // local scope
        let mut a = 20;
        a += 30;
        println!("{}", a);  // 50
    }

    println!("{}", a);  // 10
}
