use std::{rc::Rc, cell::RefCell};

fn main() {
    let a = Rc::new(RefCell::new(String::from("hoge")));
    dbg!(a.borrow()); // a.borrw() = "hoge"

    *(a.borrow_mut()) = String::from("foo");
    dbg!(a.borrow()); // a.borrow() = "foo"

    let b = Rc::clone(&a);
    *(b.borrow_mut()) = String::from("bar");
    dbg!(a.borrow()); // a.borrow() = "bar"

    let c = a.borrow_mut();
    let d = a.borrow_mut(); // panic!
}
