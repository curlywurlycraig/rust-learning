use std::ops::Deref;
use std::rc::Rc;

enum PointType {
    Big,
    Small
}

struct Point {
    point_type: PointType,
    x: i32,
    y: i32
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(inner: T) -> MyBox<T> {
        MyBox(inner)
    }

    fn move_me(self) -> MyBox<T> {
        self
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl MyBox<Point> {
    fn get_y(&self) -> i32 {
        self.0.y
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    let mut example = MyBox(Point { point_type: PointType::Small, x: 10, y: 11 });

    let other_example = Box::new(MyBox::new(String::from("Craig")));

    hello(&other_example);

    println!("{}", other_example.to_lowercase());

    other_example.move_me();
}
