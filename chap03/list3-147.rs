use std::cell::Cell;

fn main() {
    let a = Cell::new(10); // immutable object with interior mutability
    dbg!(a.get()); // a.get() = 10
    a.set(20);
    dbg!(a.get()); // a.get() = 20
}
