## The Option Enum and Its Advantages Over Null Values

This section explores a case study of `Option`, which is another enum defined by the standard library. The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

**For example:**
* If you request the first item in a non-empty list, you would get a value. 
* If you request the first item in an empty list, you would get nothing. 

Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in terms of which features you include, but the features you **exclude** are important too. 

**Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.**

> The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `Option<T>` enum is so useful that it’s even **included in the prelude**; you don’t need to bring it into scope explicitly. 

Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still ***variants*** of type `Option<T>`.

## <T> Syntax

The `<T>` syntax is a feature of Rust we haven’t talked about yet. 

It’s a generic type parameter. `<T>` means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type a different type.

Here are some examples of using Option values to hold number types and string types:

```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```

The type of some_number is `Option<i32>`. The type of `some_char` is `Option<char>,` which is a different type. Rust can infer these types because we’ve specified a value inside the Some variant. For `absent_number`, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. 

Here, we tell Rust that we mean for `absent_number` to be of type `Option<i32>`.

In general, in order to use an `Option<T>` value, you want to have code that will handle each variant. 

You want some code that will run only when you have a `Some(T)` value, and this code is allowed to use the `inner T`. 

You want some other code to run only if you have a None value, and that code doesn’t have a T value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.