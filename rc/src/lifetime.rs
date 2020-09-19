//! The book claims the following:
//!
//! We could change the definition of Cons to hold references instead,
//! but then we would have to specify lifetime parameters. By specifying
//! lifetime parameters, we would be specifying that every element in the
//! list will live at least as long as the entire list. The borrow
//! checker wouldn’t let us compile let a = Cons(10, &Nil); for example,
//! because the temporary Nil value would be dropped before a could take
//! a reference to it.
//!
//! But the following example demonstrates otherwise.

#[derive(Debug)]
enum List<'a> {
    Cons(i32, &'a List<'a>),
    Nil
}

use List::{Cons, Nil};

pub fn demo() {
    let end = Cons(1, &Cons(10, &Cons(3, &Nil)));
    let first = Cons(1, &end);
    let second = Cons(2, &end);

    take_list(first);
    take_list(second);

    println!("New list {:?}", create_list());
}

fn take_list(the_list: List) {
    println!("The list is {:?}", the_list);
}

fn create_list<'a>() -> List<'a> {
    Cons(1, &Cons(10, &Cons(3, &Nil)))
}