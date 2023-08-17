use std::{rc::Rc, cell::Cell};

fn main() {
    let a = Rc::new(Cell::new(10));
    a.set(20);
    dbg!(a.get()); // a.get() = 20

    let b = Rc::clone(&a);
    b.set(30);
    dbg!(a.get()); // a.get() = 30
}
