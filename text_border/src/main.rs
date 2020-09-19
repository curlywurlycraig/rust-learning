use std::io;

fn add_borders(input: &str) -> String {
    // create the length of the top and bottom borders
    let mut top_border = String::new();
    for _ in 0..input.len()+3 {
        top_border.push('*');
    }

    // build the result by composing the first, middle and last lines
    let input = String::from(input);
    let mut result = String::new();
    result.push_str(&top_border);
    result.push_str("\n");
    result.push_str("* ");
    result.push_str(input.trim());
    result.push_str(" *\n");
    result.push_str(&top_border);
    result.push_str("\n");

    return result;
}

fn main() {
    // get user input
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Err(err) => println!("error reading input: {}", err),
        Ok(_)    => ()
    }

    // add borders to the input
    let bordered_input = add_borders(&input);
    println!("{}", bordered_input);
}
