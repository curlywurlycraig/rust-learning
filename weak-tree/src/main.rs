use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            parent: Weak::new(),
            children: vec![]
        }
    }

    fn add_child(&mut self, node: Rc<RefCell<Node>>) {
        self.children.push(Rc::clone(&node));
    }
}

fn print_children_values(node: &Node) {
    for child in node.children.iter() {
        println!("Child value is {}", child.borrow().value)
    }
}

fn main() {
    let leaf = Node::new(3);
    let mut branch = Node::new(4);

    branch.add_child(Rc::new(RefCell::new(leaf)));

    print_children_values(&branch);

}
