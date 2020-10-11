use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(1));

    for i in 1..10 {
        let a_clone = Arc::clone(&a);
        std::thread::spawn(move || {
            let mut num = a_clone.lock().unwrap();
            *num += 1;
        });
    }

    println!("Value is {:?}", a);
}
