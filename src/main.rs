use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut tries = 5; // Define `tries` outside the loop

    loop {
        if tries == 0 {
            println!("You've run out of tries! The secret number was {secret_number}.");
            break;
        }

        println!("Please Input Your Guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                tries -= 1;
                println!("Too small! You have {tries} tries left.");
            }
            Ordering::Greater => {
                tries -= 1;
                println!("Too big! You have {tries} tries left.");
            }
            Ordering::Equal => {
                println!("You win! The secret number was {secret_number}.");
                break;
            }
        };
    }
}
