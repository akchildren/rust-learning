# Data Types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: `scalar` and `compound`.

Rust is a `statically typed language`, which means that **it must know the types of all variables at compile time**. The compiler can usually infer what type we want to use based on the value and how we use it.

## Scalar Types
A `scalar type` represents a ***single*** value. Rust has **four** primary scalar types: `integers`, `floating-point numbers`, `Booleans`, and `characters`. You may recognize these from other programming languages.

## Integer Types
An `integer` is a number without a fractional component. We used one integer type in Chapter 2, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i instead of u) that takes up `32 bits of space`. Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

For a table to show max ranges:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=d7b601e35a85b089fb1e
| Length	| Signed	| I Max                     | Unsigned  | U Max |
|-----------|-----------| -----                     |-----------|-----|
| 8-bit	    | i8	    |  127                      | u8        | 255
| 16-bit	| i16	    |  32767                    | u16       | 65535
| 32-bit	| i32	    |  2147483647               | u32       | 4294967295
| 64-bit	| i64	    |  9223372036854775807      | u64       | 18446744073709551615
| 128-bit	| i128	    |      NA                     | u128      | NA
| arch	    | isize	    |      NA                     | usize     | NA

Each variant can be either `signed` or `unsigned` and has an explicit size. 
Unsiged can be possitve or negative.
Signed will only ever be positive and are stored using two’s complement representation.

Each signed variant can store numbers from `-(2n - 1)` to `2n - 1 - 1 ` inclusive, where `n` is the number of bits that variant uses. So an `i8` can store numbers from `-(27)` to` 27 - 1`, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from `0` to `28 - 1`, which equals `0` to `255`.

Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

### Integer Literals
|Number literals	| Example       |
|-------------------|---------------|
|Decimal	        | 98_222        |
|Hex	            | 0xff          |
|Octal	            | 0o77          |
|Binary	            | 0b1111_0000   |
|Byte (u8 only)	    | b'A'          |

## Integer Overflow (Memmory issue)
Let’s say you have a variable of type `u8` that can hold values between `0` and `255`. **If** you try to change the variable to a value **outside that range**, such as `256`, **integer overflow will occur, which can result in one of two behaviors.** 
- When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. 
- Rust uses the term panicking when a program exits with an error; This is unrecoverable and will terminate the program.

### Overflow In release mode
When you’re compiling in release mode with the `--release` flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. 

In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, **the value 256 becomes 0, the value 257 becomes 1, and so on**. 

The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

To explicitly handle the possibility of **overflow**, you can use these families of methods provided by the standard library for primitive numeric types:

Wrap in all modes with the `wrapping_*` methods, such as wrapping_add.
Return the None value if there is overflow with the `checked_*` methods.
Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
Saturate at the value’s minimum or maximum values with the `saturating_*` methods.


## Floating-Point Types
Rust also has two primitive types for *floating-point numbers*, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs, it’s roughly the same speed as `f32` but is capable of more precision. **All floating-point types are signed.**

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a **single-precision float** (2.0), and `f64` has **double precision** (2.00).

### Numeric Operators
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
Each expr
```

### Booleans
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### The character type
Rust’s char type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

Note that we specify char literals with ***single quotes***, as opposed to string literals, which use double quotes. 

Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. 

Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.

# Compound Types (Different array/list styles)
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

## Tuple
A tuple is a general way of **grouping together a number of values with a variety of types into one compound type**. 

Tuples have a fixed length: **once declared, they cannot grow or shrink in size!**

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

#### Destructuring 
The variable `tup` binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

This program first creates a tuple and binds it to the variable `tup`. It then uses a pattern with `let` to take `tup` and **turn it into three separate variables.** This is called **destructuring** because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.

#### Get tuple values via indicies
We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates the tuple `x` and then accesses each element of the tuple using their respective indices. As with most programming languages, the first index in a tuple is 0.

The tuple without any values has a special name, unit. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

## The Array Type (Should use vectors instead)
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.

An array isn’t as flexible as the vector type, though. **A `vector` is a similar collection type provided by the standard library that is allowed to grow or shrink in size.** If you’re unsure whether to use an array or a vector, chances are you should use a vector.

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

#### Declaring strictness
You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]; // 5 elements all integers
```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
```rust
let a = [3; 5]; // 3,3,3,3,3
```

#### Accessing Array elements
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```