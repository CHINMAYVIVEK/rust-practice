use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    node1.borrow_mut().next = Some(node2.clone());
    println!("Singly Linked List: {:?}", node1);
}
