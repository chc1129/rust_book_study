use std::rc::Rc;

fn main() {
    let mut a = Rc::new(10); // shared reference to imuutable object
    *a = 10; // Error
}
