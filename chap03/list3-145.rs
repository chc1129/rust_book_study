use std::rc::Rc;

fn main() {
    let a = Rc::new(10);        // shared reference to immutable object
    dgb!(Rc::strong_count(&a)); // Rc::strong_count(&a) = 1

    let b = Rc:clone(&a);       // cloned shared reference
    dbg!(Rc::strong_count(&a)); // Rc::strong_count(&a) = 2
    dbg!(Rc::strong_count(&b)); // Rc::strong_count(&b) = 2
}
