use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};

pub fn demo() {
    let (first, second) = create_two_lists();
}

fn create_two_lists() -> (List, List) {
    let end = Rc::new(Cons(10, Rc::new(Nil)));
    let first = Cons(100, Rc::clone(&end));
    let second = Cons(10, Rc::clone(&end));

    (first, second)
}