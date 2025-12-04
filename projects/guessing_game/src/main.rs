use std::io;
use rand::Rng;

fn main(){
    
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("{secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess).expect("failed to read line");

    println!("You guessed {guess} ");
}