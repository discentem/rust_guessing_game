use std::io;
use std::cmp::Ordering;
use rand::Rng;

enum GuessResult {
    Small, 
    Big,
    Win,
}

fn process_guess(guess: u32, answer: u32) -> GuessResult {
    match guess.cmp(&answer) {
        Ordering::Less => return GuessResult::Small,
        Ordering::Greater => return GuessResult::Big,
        Ordering::Equal => return GuessResult::Win
    }
}

fn main() {
    let secret = rand::thread_rng().gen_range(1..100);
    println!("{secret}");
    println!("Guess the number!");
    
    loop {
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

        let g_result = process_guess(guess, secret);
        if matches!(g_result, GuessResult::Win) {
            println!("You win!");
            break;
        } else if matches!(g_result, GuessResult::Small) {
            println!("Too low! Guess again!")
        } else {
            println!("Too high! Guess again!")
        }
        
        
    }
}
