mod worker;

use std::thread;

fn main() {
    // Create an array to keep track of 5 different threads

    thread::spawn(|| {
        for i in 1..5 {
            println!("Hello thread {i} is here :)");
        }
    });

}
