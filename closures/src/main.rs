use std::thread;
use std::time::Duration;

mod lib;
use lib::Cacher;

fn calculate_expensive_result(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let mut cacher = Cacher::new(calculate_expensive_result);
    println!("Value is {}", cacher.call(10));
    println!("Value is {}", cacher.call(10));
    println!("Value is {}", cacher.call(20));
    println!("Value is {}", cacher.call(20));

    let mut closure_cacher = Cacher::new(|val| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        val * 2
    });
    println!("Value is {}", closure_cacher.call(100));
    println!("Value is {}", closure_cacher.call(100));
    println!("Value is {}", closure_cacher.call(200));
    println!("Value is {}", closure_cacher.call(200));
    println!("Value is {}", closure_cacher.call(100));
    println!("Value is {}", cacher.call(10));
    println!("Value is {}", cacher.call(20));

}
