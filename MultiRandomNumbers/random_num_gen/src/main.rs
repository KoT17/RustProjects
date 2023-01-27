use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    let mut number_of_attempts: i32 = 1;
    let mut max_val = String::new();
    let mut inputted_val = String::new();
    let mut rng = rand::thread_rng();
    let mut completed: bool;

    println!("Note: Input a negative value to leave the game");
    println!("Set an upper bound for a max random number!");
    io::stdin()
        .read_line(&mut max_val)
        .expect("Failed");
    let secret_number = rng.gen_range(1..convert_readline_to_int(max_val));

    println!("Input a value to guess [1, UpperBound]");
    io::stdin()
        .read_line(&mut inputted_val)
        .expect("Failed");

    let inputted_val = convert_readline_to_int(inputted_val);
    completed = check_number(inputted_val, secret_number, number_of_attempts);

    while !completed {
        println!("Sorry! Guess again please");
        number_of_attempts = number_of_attempts + 1;

        let mut new_val = String::new();
        io::stdin()
            .read_line(&mut new_val)
            .expect("Failed");

        let new_val = convert_readline_to_int(new_val);
        completed = check_number(new_val, secret_number, number_of_attempts);

    }

    println!("Secret number was: {secret_number}");
    println!("Number of attempts: {number_of_attempts}")
}

fn check_number(val: i32, secret_number: i32, attempts: i32) -> bool {
    let mut completed = true & false | false;
    if val < 0 {
        completed = true;
        println!("Sad to see you go :(");
    }

    match val.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
            if attempts==1 {
                println!("Congrats! You got it on the first try!");
            } else {
                println!("You finally narrowed it down, congrats!")
            }
            completed = true;
            },
    }
    return completed;
}

fn convert_readline_to_int(str: String) -> i32 {
    return str.trim().parse::<i32>().unwrap();
}