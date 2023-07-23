# Loops
It’s often useful to execute a block of code more than once. For this task, Rust provides several ***loops***, which will run through the code inside the loop body to the end and then start immediately back at the beginning.

## Repeating Code with loop
The `loop` keyword tells Rust to execute a block of code over and over again **forever** or until you explicitly tell it to stop.

```rust
fn main() {
    // This will loop forever 
    loop {
        println!("again!");
    }
}
```

### Returning Values from loops
One of the uses of a `loop` is to *retry an operation you know might fail*, such as checking whether a thread has completed its job. 

You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### Loop Labels to Disambiguate Between Multiple Loops
If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. 

You can optionally specify a ***loop label*** on a `loop` that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:

```rust
fn main() {
    let mut count = 0;
    // Define a loop label to explain what a loop is doing (LOVE THIS OMG <3)
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

The outer loop has the label `'counting_up`, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first `break` that doesn’t specify a label will exit the inner loop only. The `break 'counting_up; `statement will exit the outer loop.

## While Loops
A program will often need to evaluate a condition within a loop. While the condition is `true`, the loop runs. When the condition ceases to be `true`, the program calls `break`, stopping the loop. 

It’s possible to implement behavior like this using a combination of loop, if, else, and break; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a `while` loop. In Listing 3-3, we use while to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break`, and **it’s clearer**. While a condition evaluates to true, the code runs; otherwise, it exits the loop.

## For loop
```rust 
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust. 

### Foreach Equivalent
Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop. 

The way to do that would be to use a `Range`, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

```rust
fn main() {
    // Rev stands for the ravers (i.e. start from 4 backwards)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```