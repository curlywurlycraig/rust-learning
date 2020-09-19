fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3];

    let is_a = move |x: Vec<i32>| {x == a};

    println!("Is it the same as a? {}", is_a(vec![1,2,3]));

    println!("Is it the same as a? {}", is_a(vec![2,3,4]));
}
