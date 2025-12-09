use rand::prelude::*;
use std::io;

fn main() {
    println!("Enter a Random Number");
    
    let mut rng = rand::rng();
    let r: u32 = rng.random_range(1..=100);

    let mut g = String::new();
    io::stdin().read_line(&mut g).expect("Failed to read line");

    





    
}




