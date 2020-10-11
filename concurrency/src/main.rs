use std::thread;
use std::sync::mpsc;
use std::time::Duration;

struct MyMessage {
    message: String
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in vec![1,2,3] {
            tx.send(MyMessage {
                message: String::from(format!("{}", i))
            }).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got a message! {}", received.message)
    }
}
