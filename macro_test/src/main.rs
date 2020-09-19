fn main() {
    // macro expansion works on (), [], or {}
    println!("I wonder if this prints");
    println!["I wonder if this prints"];
    println!{"I wonder if this prints"};
}
