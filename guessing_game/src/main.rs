use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess: ");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    while true {
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess_int = guess.parse::<i32>().unwrap();
        
        if guess_int > secret_number {
            println!("{guess} is too large");
        }
        else if guess_int < secret_number {
            println!("{guess} is too small")
        }
        else {
            println!("You guessed correctly!")
        }
    }

    println!("The secret number is: {secret_number}");
}
