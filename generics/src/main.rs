use std::cmp::Ordering;

trait Magnitude {
    fn magnitude(&self) -> f32 { 0f32 }
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl Magnitude for Point<&str, &str> {}

impl Magnitude for Point<f32, f32> {
    fn magnitude(&self) -> f32 {
        (self.x.powf(2f32) + self.y.powf(2f32)).sqrt()
    }
}

impl PartialEq for Point<f32, f32> {
    fn eq(&self, other: &Self) -> bool {
        self.magnitude() == other.magnitude()
    }
}

impl PartialOrd for Point<f32, f32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl<T, U> Point<T, U>
where T: Copy, U: Copy
{
    fn mixup<V, W>(&self, other: &Point<V, W>) -> Point<T, W>
    where V: Copy, W: Copy
    {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn sum_magnitudes<T, U>(first: &T, second: &U) -> f32
where T: Magnitude, U: Magnitude
{
    first.magnitude() + second.magnitude()
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = &item;
        }
    }

    max
}

fn main() {
    let first = Point { x: 5f32, y: 10f32 };
    let second = Point { x: "Craig", y: "Wilkinson" };

    let mixed = first.mixup(&second);
    println!("First: {:?}", first);
    println!("Second: {:?}", second);
    println!("Mixed: {:?}", mixed);

    println!("Magnitude of the first point: {}", first.magnitude());
    println!("Magnitude of the second line: {}", second.magnitude());

    println!("Sum of both magnitudes: {}", sum_magnitudes(&first, &first));

    println!("The max of 1, 5, 10, and 120 is {}", largest(&vec![1, 5, 120, 10]));

    let big_point = Point { x: 5f32, y: 5f32 };
    let small_point = Point { x: 2.5f32, y: 5f32};
    println!("The bigger point is {:?}", largest(&vec![big_point, small_point]))
}
