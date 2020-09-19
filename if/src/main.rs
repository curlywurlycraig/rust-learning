fn main() {
    let x = 10;

    let y = if x == 10 {
        println!("x is 10");
        10
    } else if x == 5 {
        println!("x is 5");
        5
    } else {
        println!("x isn't 5 or 10 :(");
        1
    };

    println!("y is {}", y);
}
