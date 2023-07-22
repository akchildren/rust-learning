use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io;
use std::ops::RangeInclusive;
use colored::*;

//The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.
const MIN_RANGE: u32 = 1; 
const MAX_RANGE: u32 = 55; 
const RANGE: RangeInclusive<u32> = MIN_RANGE..=MAX_RANGE;


fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(RANGE);
    let mut counter: u32 = 0;

    loop {
        println!("Please input your guess. It must be between {MIN_RANGE} - {MAX_RANGE}");

        // Mutable varriable (Can be)
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
        counter +=1;

        //comparing and ordering values. (CMP can be used as comparitor)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("{} You took {counter} attempts !","You win!".green());
                break;
            }
        }
    }
}
