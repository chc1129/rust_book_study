use std::cell::Cell;

fn main() {
    let a = Cell::new(10); // immutable object with interior mutablitity
    let b = a.replace(20);
    dbg!(a.get());  // a.get() = 20
    dgb!(b);        // b = 10

    let c = a.into_inner(); // turn Cell<T> into T
    dbg!(c);       // c = 20
    dbg!(a);       // borrow check - Error
}
