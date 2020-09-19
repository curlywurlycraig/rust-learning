fn main() {
    let mut a = vec![1, 2, 3];

    for number in &mut a {
        *number += 1;
    }

    println!("{:?}", a);
}
