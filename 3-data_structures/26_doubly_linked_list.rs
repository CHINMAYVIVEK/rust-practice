use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct DNode {
    data: i32,
    prev: Option<Rc<RefCell<DNode>>>,
    next: Option<Rc<RefCell<DNode>>>,
}

impl DNode {
    fn new(data: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DNode {
            data,
            prev: None,
            next: None,
        }))
    }
}

fn main() {
    let d1 = DNode::new(10);
    let d2 = DNode::new(20);
    d1.borrow_mut().next = Some(d2.clone());
    d2.borrow_mut().prev = Some(d1.clone());
    println!("Doubly Linked List: {:?}", d1);
}
