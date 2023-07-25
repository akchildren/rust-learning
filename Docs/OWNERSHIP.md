# Ownership
https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

Note: Refer to https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move when trying to understand pointers.

> Promotes using the stack over the use of the heap memory

`Ownership` is Rust’s most unique feature and has deep implications for the rest of the language. **It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works.** 

***Ownership* is a set of rules that govern how a Rust program manages memory.** 

All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. 

Rust uses a third approach: **memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.** None of the features of ownership will slow down your program while it’s running.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. 

### Ownership rules
* Each value in Rust has an `owner`.
* There can only be **one** `owner` at a time.
* When the `owner` *goes out of scope*, the `value` will be **dropped**.

### Varriable Scope
As a first example of ownership, we’ll look at the scope of some variables. A scope is the range within a program for which an item is valid. Take the following variable:

```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```
The variable `s` refers to a `string literal`, where the value of the string is hardcoded into the text of our program.

The variable is valid from the point at which it’s declared until the end of the **current** *scope*. 

### The String Type
To illustrate the rules of ownership, we need a data type that is more complex.

We want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the `String` type is a great example.

String literals are convenient, but they aren’t suitable for every situation in which we may want to use text.  Reasons:
 
* They’re `immutable`. 
* Not every string value can be known when we write our code: for example, what if we want to take user input and store it? 

For these situations, Rust has a second string type, `String`. *This type manages data allocated on the heap* and **as such is able to store an amount of text that is unknown to us at compile time**. You can create a String from a string literal using the from function, like so:

```rust
let s = String::from("hello");
```

This kind of string can be mutated:
```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

> String be mutated but literals cannot? The difference is in how these two types deal with memory.

### Memory and Allocation
In the case of a string literal, we know the contents at compile time, so the text is **hardcoded** directly into the final executable. 

This is why **string literals are fast and efficient.** But these properties only come from the string literal’s `immutability`. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the `String` type, in order to support a `mutable`, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 

**This means:**

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. 

In most languages without a `GC`, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. 

> Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one `allocate` with exactly one `free`.

Rust takes a different path: the **memory is automatically returned once the variable that owns it goes out of scope**. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:
```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope. 

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

### Variables and Data Interacting with Clone
If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

> When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. **It’s a visual indicator that something different is going on.**

### Stack-Only Data: Copy

```rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```
This code seems to `contradict` what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

The reason is that types such as `integers` that have a known size at compile time are stored entirely on the stack, so copies of the actual values are *quick to make*. 

That means there’s no reason we would want to prevent `x` from being valid after we create the variable `y`. In other words, *there’s no difference between deep and shallow copying here*, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.