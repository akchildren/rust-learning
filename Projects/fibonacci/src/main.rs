use std::io;
use colored::*;

fn main() {
    println!("{}", "Please enter a number to find the nth Fibonacci number".green());

    // Mutable varriable (Can be)
    let mut position = String::new();

    // Standard input from user with assertion attached for error handling
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");

    let position: u32 = position.trim().parse().expect("Please type a number!");

    println!("The fibonacci value of {}th position is {}", position, fibonacci(position)); 
}

fn fibonacci(n: u32) -> u32 {
    let mut total = 0;
    let mut n2: u32 = 1;
    
    for _number in 0..=n {
        let prev: u32 = total;
        total += n2;
        n2 = prev;
    }
    
    total
}