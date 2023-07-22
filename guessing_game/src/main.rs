use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Standard input from user with assertion attached for error handling
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Cast String to u32 (unsigned integer 32 bit)
        // This is done via [: u32]
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Result returns OK or Err based on conditional (Result is an inbuilt enum)

        println!("You guessed: {guess}");
        
        //comparing and ordering values. (CMP can be used as comparitor)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}