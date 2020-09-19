struct my_container {
    value: i32
}

impl Iterator for my_container {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;

        match self.value {
            100 => None,
            _ => Some(self.value)
        }
    }
}

fn main() {
    let my_fun_iterator = my_container {
        value: 50
    };

    for i in my_fun_iterator {
        println!("It is {}", i);
    }
}