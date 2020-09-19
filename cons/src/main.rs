#[derive(Debug)]
enum List<T: Copy> {
    Cons(T, Box<List<T>>),
    Nil
}

impl<T: Copy> List<T> {
    fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self
        }
    }
}

struct ListIter<'a, T: Copy> {
    current: &'a List<T>
}

impl<'a, T: Copy> Iterator for ListIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Nil => None,
            Cons(a, rest) => {
                self.current = rest;
                Some(*a)
            }
        }
    }
}

use List::Cons;
use List::Nil;

fn main() {
    let my_list = Cons(1, Box::new(Cons(10, Box::new(Nil))));

    for i in my_list
        .iter()
        .map(|val| val * 10)
    {
        println!("it is {}", i);
    }

    println!("I still have my list! {:?}", my_list);
}
