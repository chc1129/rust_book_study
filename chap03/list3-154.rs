use std::rc::Rc;
use std::cell::ReCell;

(...略...)

fn main() {
    let a = Node::new(Hoge {});
    let b = Node::new(Hoge {});
    a.borrow_mut().next = Some(b.clone()); // 追加
    b.borrow_mut().next = Some(a.clone()); // 追加
}
