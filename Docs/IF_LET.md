# Concise Control Flow with if let

> If you don't want to handle none for a good reason, then this is probably the correct thing to use

> **Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.**

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest. 

Consider the following that matches on an `Option<u8>` value in the `config_max` variable but only wants to execute code if the value is the Some variant.

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
```

If the value is `Some`, we print out the value in the `Some` variant by binding the value to the variable max in the pattern. 

We don’t want to do anything with the `None` value. To satisfy the match expression, we have to `add _ => ()` after processing just one variant, which is annoying boilerplate code to add.

> **Instead, we could write this in a shorter way using `if let`. The following code behaves the same as the match**

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

The syntax `if let` takes a pattern and an expression separated by an equal sign. 

It works the same way as a match, where the expression is given to the `match` and the pattern is its first arm. 

In this case, the pattern is `Some(max)`, and the `max` binds to the value inside the Some. We can then use `max` in the body of the `if let` block in the same way we used `max` in the corresponding `match` arm. The code in the `if let` block isn’t run if the value doesn’t match the pattern.

### Using else in if let
We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the `_` case in the `match` expression that is equivalent to the if let and else. 

Recall the Coin enum definition in MATCH.md, where the Quarter variant also held a UsState value. If we wanted to count all non-quarter coins we see while also announcing the state of the quarters, we could do that with a match expression, like this:

```rust
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
```

**Or we could use an if let and else expression, like this:**
```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```