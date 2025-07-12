use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to the guessing game!");
   println!("__________________________________") ;
    loop {
    println!("Guess a number from 1 to 100:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Please use a number");
    let guess:u8 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
        };

    match guess.cmp(& secret_number){
        Ordering::Less => println!("Too Low guess again! "),
        Ordering::Greater => println!("Too High guess again! "),
        Ordering::Equal => {
                println!("You win!");
                break;
            }
    };
    }
}
