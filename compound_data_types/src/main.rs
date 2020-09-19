fn main() {
    // tuple
    let x = (1, "hello");
    println!("{:?}", x);

    // destructuring 'let'
    let (a, b, c) = (1, 2, "hi");
    println!("({}, {})", a + b, c);

    let x = 1;
    let y = x;
    println!("{}", x);

    // destructuring from a function result
    let (d, e) = next_two(2);
    println!("next two numbers after 2 are {} and {}", d, e);
}

// a function which returns multiple values in the form of a tuple
fn next_two(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}
