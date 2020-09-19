fn main() {
    let x = 10;
    let result = add_one(x);
    println!("{} + 1 = {}", x, result);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
