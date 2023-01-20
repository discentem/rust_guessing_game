use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn process_guess(guess: u32, answer: u32) -> &'static str {
    match guess.cmp(&answer) {
        Ordering::Less => return "Too small",
        Ordering::Greater => return "Too big",
        Ordering::Equal => return "You win!"
    }
}

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

        let g_result = process_guess(guess, secret);
        println!("{}", g_result);
        if g_result == "You win!" {
            break;
        }
        
        
    }
}
