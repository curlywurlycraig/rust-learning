// &str are immutable!

fn main() {
    let mut some_string = String::from("Example");
    let first_four_chars: &str = &some_string[..4];

    let array_example: [u8; 4] = [1, 2, 3, 4];
    let array_slice: &[u8] = &array_example[..2];

    let mut thing: [String; 4] = [String::from(""), String::from("a"), String::from("b"), String::from("c")];
    thing[0] = String::from("test");

    println!("First string is {}", thing[0]);
    thing[0] = String::from("test2");
    println!("First string is now {}", thing[0]);

    let mut first_char = &first_four_chars[0..1];
    first_char = "a";
    let number_of_characters = first_char.len();

    println!("first four is now {}", first_four_chars);

    // The following commented out code wouldn't be valid, because
    // the slice is an immutable reference. We can't have both an
    // immutable and a mutable reference.
    // some_string.replace_range(1..3, "e");

    println!("Slice of length {} is {}", number_of_characters, first_char);
}
