fn main() {
    println!("Please enter a temperature in farenheit.");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line.");

    let input: f32 = input.trim().parse().expect("That doesn't look like a valid number!");
    let result = farenheit_to_celcius(input);

    println!("Result is {}", result);
}

fn farenheit_to_celcius(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}