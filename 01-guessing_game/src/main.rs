use std::io;

use rand::{thread_rng, Rng};
use std::cmp::Ordering;


fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number: {secret_number}");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read from stdin");
    
        println!("guess: {}", guess);
        // let i32_guess: i32 = guess.trim().parse().expect("failed to parse to i32");
        let i32_guess = match guess.trim().parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                println!("failed to parse");
                continue;
            }       
        };

        match i32_guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("guess is equal with secret number");
                break
            },
            Ordering::Greater => println!("guess is greater than secret number"),
            Ordering::Less => println!("guess is lesser than secret number"),
        }    
    }
}
