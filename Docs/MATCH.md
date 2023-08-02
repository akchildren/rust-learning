# The match Control Flow Construct
Rust has an extremely powerful control flow construct called `match` that allows you to compare a **value** *against* a series of patterns and then execute code based on which pattern matches. 

Patterns can be made up of literal values, variable names, wildcards, and many other things

### Example

**Think of a `match` expression as being like a coin-sorting machine:**
- Coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. 
- In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

Speaking of coins, let’s use them as an example using `match`! 

We can write a function that takes an unknown US coin and, in a similar way as the counting machine, determines which coin it is and returns its value in cents.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Example breakdown
Let’s break down the `match` in the `value_in_cents` function. 

First we list the `match` keyword followed by an expression, which in this case is the value `coin`. 

This seems very similar to a conditional expression used with if, but there’s a big difference: with if, the condition needs to evaluate to a Boolean value, **but here it can be any type**. The type of coin in this example is the Coin enum that we defined on the first line.

Next are the `match` arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value `Coin::Penny` and then the `=>` operator that separates the pattern and the code to run. The code in this case is just the value `1`. Each arm is separated from the next with a comma.

When the match expression executes, it compares the resultant value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed.

> The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire `match` expression.

#### Match with brackets (rare but good to know)

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns That Bind to Values

**Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.** This is how we can extract values out of enum variants.

As an example, let’s change one of our enum variants to hold data inside it.

From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have this extra value. We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Let’s imagine that a friend is trying to collect all 50 state quarters. While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so that if it’s one our friend doesn’t have, they can add it to their collection.

In the match expression for this code, we add a variable called `state` to the pattern that matches values of the variant `Coin::Quarter`. When a `Coin::Quarter` matches, the `state` variable will bind to the value of that quarter’s state. Then we can use `state` in the code for that arm, like so:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

#### How Coin::Quarter(state) is satisfied

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` would be `Coin::Quarter(UsState::Alaska)`. 

When we compare that value with each of the match arms, none of them match until we reach `Coin::Quarter(state)`. At that point, the binding for `state` will be the value `UsState::Alaska`. We can then use that binding in the `println!` expression, thus getting the inner state value out of the `Coin` enum variant for `Quarter`.

## Matching with Option\<T\>

In the previous section, we wanted to get the inner `T` value out of the `Some` case when using `Option<T>`; we can also handle `Option<T>` using `match`, as we did with the Coin enum! 

Instead of comparing coins, we’ll compare the variants of `Option<T>`, but the way the match expression works remains the same.

Let’s say we want to write a function that takes an `Option<i32>` and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the `None` value and not attempt to perform any operations.

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

### Breakdown

Let’s examine the first execution of `plus_one` in more detail. When we call `plus_one(five)`, the variable x in the body of `plus_one` will have the value Some(5). We then compare that against each match arm:

```rust
            None => None,
```

The `Some(5)` value doesn’t match the pattern `None`, so we continue to the next arm:
```rust
            Some(i) => Some(i + 1),
```
Does `Some(5)` match `Some(i)`? It does! We have the same variant. The `i` binds to the value contained in Some, so `i` takes the value `5`. The code in the match arm is then executed, so we add `1` to the value of `i` and create a new `Some` value with our total `6` inside.

> **Matches Are Exhaustive** We must always handle none if matching an option type

## Catch-all Patterns and the `_` Placeholder (Default action)

Using enums, we can also take special actions for a few particular values, but for all other values take one **default** action. 

Imagine we’re implementing a game where, if you roll a 3 on a dice roll, your player doesn’t move, but instead gets a new fancy hat. 

If you roll a 7, your player loses a fancy hat. For all other values, your player moves that number of spaces on the game board. 

Here’s a `match` that implements that logic, with the result of the dice roll hardcoded rather than a random value, and all other logic represented by functions without bodies because actually implementing them is out of scope for this example:

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

#### `_` does not always have to do anything 

```rust
    let dice_roll = 9;
    // We can use an empty tuple to represent do nothing
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```
Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.