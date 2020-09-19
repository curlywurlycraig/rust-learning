use std::io;

fn parse_expression(input: &str) -> i32 {
    let characters = String::from(input).characters();
    let first_char = characters.nth(0);

    match characters.nth(0) {
        '+' => 
    }
    // if it's a number, just return that
    match characters.nth(0).parse() {
        Ok(num) => num,
        Err(err) => {
            
        }
    }
}

fn main() {
    let mut input = String::new();

    loop {
        // get the user input
        io::stdin().read_line(&mut input);

        characters = input.chars();

        for character 
    }
}
