use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
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

struct Hoge;

fn main() {
    let a = Node::new(Hoge {});
    let b = Node::new(Hoge {});
}
