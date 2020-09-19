fn main() {
    println!("Enter a number, and I will tell you which fibonnaci number that is.");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");

    let parsed_input: i128 = input.trim().parse().expect("Failed to parse input.");

    let result = fibo(parsed_input);
    println!("The number is: {}", result);

}

fn fibo(n: i128) -> i128 {
    let mut last = 1;
    let mut current = 1;
    let mut tmp;

    for _ in 0..n {
        tmp = current;
        current = last + current;
        last = tmp;
    }

    current
}