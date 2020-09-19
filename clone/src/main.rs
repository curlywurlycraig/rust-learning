fn main() {
    let mut example = String::from("content");
    let example_2 = example.clone();
    example += "1";

    println!("{}, {}", example, example_2);
}
