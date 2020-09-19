use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, String> = HashMap::new();
    hm.insert(1, String::from("asdf"));
    hm.entry(1).or_insert(String::from("test"));

    assert_eq!(hm.get(&1).unwrap(), "asdf");

    // Count the occurrences of words in a sentence
    let example: &str = "i wonder o i wonder";
    let mut token_counts: HashMap<&str, u32> = HashMap::new();

    for token in example.split_whitespace() {
        let count = token_counts.entry(token).or_insert(0);
        *count += 1;
    }

    println!("The tokens are: {:?}", token_counts);

    println!("The mean of 3, 7, and 9 is {}", mean(&vec![3, 7, 9]));
    println!("The median of 3, 7, and 9 is {}", median(&vec![3, 7, 9]));
    println!("The median of 3, 7, 8, and 9 is {}", median(&vec![3, 7, 8, 9]));

    let vec_which_should_not_change = vec![3, 7, 7, 8, 9];
    println!("The mode of 3, 7, 7, 8, and 9 is {}", mode(&vec_which_should_not_change));
    println!("That vec looks like {:?}", vec_which_should_not_change);
}

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for i in v.iter() {
        sum += *i;
    }

    sum as f32 / (v.len() as f32)
}

fn median(v: &Vec<i32>) -> f32 {
    let mut cloned: Vec<i32> = v.clone();
    cloned.sort();

    match cloned.len() % 2 {
        0 => {
            let lower = cloned.get((cloned.len() / 2) - 1).unwrap();
            let higher = cloned.get(cloned.len() / 2).unwrap();
            (*higher as f32 + *lower as f32) / 2f32
        },
        _ => *cloned.get((cloned.len() - 1) / 2).unwrap() as f32
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, u32> = HashMap::new();
    let mut max_count = 0;
    let mut result: i32 = *v.get(0).unwrap();

    for &i in v.iter() {
        let count: &mut u32 = counts.entry(i).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            result = i;
        }
    }

    result
}