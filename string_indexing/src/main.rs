fn char_at(s: &String, n: usize) -> char {
    // get the third character in the string object
    match s.chars().nth(n) {
        Some(n) => n,
        None => '?'
    }
}

fn main() {
    // create a String from a primitive
    let string_object = "hello there".to_string();

    // as an iterator through characters
    let characters = string_object.chars();
    println!("string object: {}", string_object);
    for character in characters {
        println!("character: {}", character);

        match character {
            'h' => println!("that's an h!"),
            _   => ()
        }
    }

    println!("third character: {}", char_at(&string_object, 3));
}
