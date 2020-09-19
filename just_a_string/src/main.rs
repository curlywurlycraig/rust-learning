fn main() {
    // String literal. Allocated on the stack.
    // Has type &str because it's a reference to a string with fixed length.
    // Must be a reference because the compiler must know then size of the allocation at compile time.
    let mut example_string: &str = "yes";
    example_string = "no";
    println!("{}", example_string);

    // String type is allocated on the heap
    let mut another_string = String::from("also yes");
    another_string += " and again";
    println!("{}", another_string);
}

fn example(a: &str) {
    println!("{}", a)
}