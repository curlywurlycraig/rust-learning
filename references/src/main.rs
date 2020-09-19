fn main() {
    let my_string: String = String::from("example string haha");

    let first_word: &str = first_item(words(&my_string));

    println!("First word is '{}'", first_word);
}

fn words(s: &str) -> Vec<&str> {
    let bytes: &[u8] = s.as_bytes();

    let mut result = Vec::new();

    let mut last_space = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            result.push(&s[last_space..i]);
            last_space = i+1;
        }
    }

    result.push(&s[last_space..s.len()]);

    result
}

fn first_item<T: Copy>(v: Vec<T>) -> T {
    v[0]
}