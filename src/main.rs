use std::io;

use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to the guessing game!");
   println!("__________________________________") ;
    println!("Guess a number from 1 to 100:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Please use a number");
    println!("{guess}");
    println!("{secret_number}");
}
