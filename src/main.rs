use std::io;

fn main() {
    println!("Welcome to the guessing game!");
   println!("__________________________________") ;
    println!("Guess a number from 1 to 100:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Please use a number");
    println!("{guess}");
}
