use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, rest) => Some(rest),
            Nil => None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
