fn main() {
    // Reading elements
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(10);

    let first: &i32 = &v[0];

    println!("The first element is {}", first);

    match v.get(0) {
        Some(x) => println!("The first element is {}", x),
        None => println!("There is no first element.")
    }

    // Iterating over elements
    let mut iter_v = vec![100, 22, 33];
    for i in &iter_v {
        println!("Elem: {}", i);
    }

    for i in &mut iter_v {
        *i += 10;
    }

    for i in &iter_v {
        println!("Elem is now {}", i);
    }

    // Using an enum to store different types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let mut mixed_v = vec!(
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(10.5),
        SpreadsheetCell::Text(String::from("Example"))
    );

    for val in &mixed_v {
        println!("A mixed value: {:?}", val);
    }
}
