use std::{io, process};
use rand::Rng;

fn main() {
    println!("Input the upper bound of the random number");
    let mut max_val = String::new();
    io::stdin().read_line(&mut max_val).expect("Failed to read line");
    let max_val = convert_readline_to_int(max_val);


    let handle = std::thread::spawn(move || {
        for _ in 1..10 {
            run(max_val);
        }
    });

    handle.join().expect("Threads failed");// Wait for the threads to finish

    run(max_val);
}

fn run(max_val: i32) {

    let mut number_of_attempts: i32 = 0;
    let mut rng = rand::thread_rng();
    let mut completed: bool = false;
    let process_id = process::id().to_string();

    println!("Secret Number has been picked for Thread {}",process_id);
    let secret_number = rng.gen_range(1..max_val);

    while !completed {
        number_of_attempts = number_of_attempts + 1;
        let new_guess = rng.gen_range(1..max_val);
        completed = check_number(new_guess, secret_number);
    }

    println!("Secret number for Thread {process_id} was : {secret_number}");
    println!("Number of attempts for Thread {process_id}: {number_of_attempts}")
}

fn check_number(val: i32, secret_number: i32) -> bool {
    return val == secret_number;
}

fn convert_readline_to_int(str: String) -> i32 {
    return str.trim().parse::<i32>().unwrap();
}