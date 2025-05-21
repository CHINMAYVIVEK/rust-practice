use std::boxed::Box;

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(&mut self, data: i32) {
        self.root = Self::insert_node(self.root.take(), data);
    }

    fn insert_node(node: Option<Box<Node>>, data: i32) -> Option<Box<Node>> {
        match node {
            Some(mut n) => {
                if data < n.data {
                    n.left = Self::insert_node(n.left.take(), data);
                } else {
                    n.right = Self::insert_node(n.right.take(), data);
                }
                Some(n)
            }
            None => Some(Box::new(Node {
                data,
                left: None,
                right: None,
            })),
        }
    }

    fn inorder(&self) {
        fn traverse(node: &Option<Box<Node>>) {
            if let Some(n) = node {
                traverse(&n.left);
                print!("{} ", n.data);
                traverse(&n.right);
            }
        }
        traverse(&self.root);
        println!();
    }
}

fn main() {
    let mut bst = BST::new();
    bst.insert(8);
    bst.insert(3);
    bst.insert(10);
    bst.insert(1);
    println!("BST Inorder Traversal:");
    bst.inorder();
}
