# References and Borrowing

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

The issue with the tuple code in Listing above is that we have to return the `String` to the *calling function* so we can still use the String after the call to calculate_length, because the String was moved into calculate_length. 

> Instead, we can provide a `reference` to the `String` value. 

## Reference

A `reference` is like a pointer in that it’s an **address we can follow to access the data stored at that address; that data is owned by some other variable.**

Unlike a `pointer`, a `reference` is **guaranteed** *to point to a valid value of a particular type for the life of that reference.*

Here is how you would define and use a *calculate_length* function that has a `reference` to an object as a parameter instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, note that we pass `&s1` into calculate_length and, in its definition, we take `&String` rather than String. 

**These ampersands represent `references`**, and they allow you to refer to some value without taking ownership of it.

> **Note**: The opposite of referencing by using `&` is `dereferencing`, which is accomplished with the dereference operator, *. 

The `&s1` syntax lets us create a `reference` that *refers* to the value of `s1` **but does not own it**. 

> Because the `reference` does not own it, the value the `statement` points to will not be dropped when the `reference` stops being used.

Likewise, the signature of the function uses `&` to indicate that the type of the parameter s is a reference. 

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```

The scope in which the variable `s` is valid is the same as any function parameter’s scope, but the value pointed to by the reference is not dropped when `s` stops being used, because `s` doesn’t have ownership. 

## Borrowing

When functions have `references` as `parameters` *instead of the actual values*, **we won’t need to return the values in order to give back ownership, because we never had ownership.**

> We call the action of creating a reference `borrowing`. As in real life, if a person owns something, you can borrow it from them. *When you’re done, you have to give it back. You don’t own it.*

If we try to modify something we’re borrowing, it will throw an error. **Just as variables are immutable by default**, so are references. *We’re not allowed to modify something we have a reference to.*

### Mutable References

To modify a borrowed value, we will require a mutable reference.
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First we change `s` to be `mut`. Then we create a mutable reference with `&mut s` where we call the change function, and update the function signature to accept a mutable reference with some_string:` &mut String`. This makes it very clear that the change function will mutate the value it borrows.

### Not allowed two mutable references to a value (one owner concept)

`Mutable` references have one big restriction: if you have a `mutable reference` to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

Example failure scenario
```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

Error Response: `error[E0499]: cannot borrow s as mutable more than once at a time
 --> src/main.rs:5:14`

**This error says that this code is invalid because we cannot borrow `s` as `mutable` more than once at a time**. The first mutable borrow is in `r1` and must *last until it’s used in the println!*, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.

It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. 

> **The benefit of having this restriction is that Rust can prevent data races at compile time.** 

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more `pointers` access the same data at the same time.
- At least one of the `pointers` is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data

```diff
- Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!**
```

**BAD EXAMPLE**
```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG f***ing PROBLEM, abort! YOU CANNOT HAVE A MUTABLE REFERENCE ALONG SIDE A IMUTABLE ONE!!!!

    println!("{}, {}, and {}", r1, r2, r3);
```

**WORKING EXAMPLE**
```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point as reference has been dropped

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

### Dangling References
In languages with pointers, it’s easy to erroneously create a ***dangling pointer***—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. 

>**In Rust, by contrast, the compiler guarantees that references will never be dangling references**

BAD EXAMPLE
```rust
fn main() {
    let reference_to_nothing = dangle(); // Its borrowing but not using?? So it's dangled
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s 
}
```

GOOD EXAMPLE
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```