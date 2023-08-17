use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = a.clone();

    println!("{} {}", a, b);
}
