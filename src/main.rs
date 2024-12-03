use std::io;
fn main() {
    println!("Guess The Number");

    println!("Please Input Your Guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You Guess {guess}");
}
