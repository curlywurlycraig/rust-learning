use std::ops::{Deref, DerefMut};
use std::fmt::Display;

struct MyTest {
    thing: i32
}

impl Deref for MyTest {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &1
    }
}

impl DerefMut for MyTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thing
    }
}

impl Dere

#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Going out of scope! {}", self.0);
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    {
        let thing = MyBox::new(100);
    }

    hello(&mut MyBox::new(String::from("Craig")));
}
