fn main() {
    let result: i32 = "42".parse().expect("Failed to parse number.");
    println!("{}", result);

    // Fails: the fromstr trait isn't implemented for (i32, i32, i32)
    let result: (i32, i32, i32) = "(1, 2, 3)".parse().expect("Failed to parse tuple.");
    println("{}", result);
}