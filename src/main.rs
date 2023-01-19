use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..100);
    println!("{secret}");
    
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut your_guess = String::new();

        io::stdin()
            .read_line(&mut your_guess)
            .expect("Failed to read line");

        println!("You guessed: {your_guess}");

        let guess: u32 = match your_guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
