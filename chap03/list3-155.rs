use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Link<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node <T> {
    elem: T,
    next: Link<T>,
}
impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
        }))
    }
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("destructed.");
    }
}

#[derive(Debug)]
struct Hoge {
    value: i32,
}

fn main() {
    let a = Node::new(Hoge { value: 10 });
    let b = Node::new(Hoge { value: 20 });
    a.borrow_mut().next = Some(Rc::downgrade(&b));
    b.borrow_mut().next = Some(Rc::downgrade(&a));
    {
        let mut a_ref = a.borrow_mut();
        if let Some(next) = a_ref.next.take() {
            if let Some(node) = next.upgrade() {
                println!("{}", node.borrow().elem.value);
            }
            a_ref.next = Some(next);
        }
        println!("{:?}", a_ref);
    }
    {
        let mut b_ref = b.borrow_mut();
        if let Some(next) = b_ref.next.take() {
            if let Some(node) = next.upgrade() {
                println!("{}", node.borrow().elem.value);
            }
            b_ref.next = Some(next);
        }
        println!("{:?}", b_ref);
    }
}
