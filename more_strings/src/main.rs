fn main() {
    // Two ways of getting the same String
    let a_str: String = "asdf".to_string();
    let b_str: String = String::from("asdf");

    // A way of building a String
    let mut c_str: String = String::new();
    c_str.push_str("asdf");
}
