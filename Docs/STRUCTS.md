# Structs
> A `struct`, or `structure`, is a custom data type that lets you **package** together and name multiple related values that make up a meaningful group. 

> A struct is like an object’s data attributes

### Defining and Instantiating Structs
Structs are similar to `tuples`, discussed in “The Tuple Type” section, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. 

Unlike with tuples, in a struct you’ll **name** each piece of data so it’s *clear* what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. 

>A struct’s name should describe the significance of the pieces of data being grouped together. 

Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. 

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Using a struct
To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. 

We create an instance by stating the name of the struct and then add curly brackets containing key: *value* pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. 

**We don’t have to specify the fields in the same order in which we declared them in the struct.** 

In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

### Retrieving field from struct
To get a specific value from a struct, we use dot notation. For example, to access this user’s email address, we use `user1.email`. 

If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

### Using Field Init shorthand
We can use the field init shorthand syntax to rewrite build_user so it behaves exactly the same but doesn’t have the repetition of `username` and `email`

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // key will be param name and value will be param value
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances from Other Instances with Struct Update Syntax
It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using *struct* update *syntax*.

```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Using struct update syntax above, we can achieve the same effect with less code, as shown in the example below. 
> **The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.**

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

This creates an instance in `user2` that has a different value for `email` but has the field values for the rest of the struct object. 

The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

#### Important to note: We would not be able to use user1 again as this has been transfered to user2
Note that the struct update syntax uses `=` like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section. 

In this example, **we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2**. If we had given `user2` new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. 

Both active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

### Using Tuple Structs Without Named Fields to Create Different Types
Rust also supports structs that look similar to tuples, called tuple structs. 

`Tuple structs` have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. 

> Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-Like Structs Without Any Fields
You can also define structs that don’t have any fields! These are called `unit-like structs` because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. 

Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

To define `AlwaysEqual`, we use the `struct` keyword, the name we want, and then a semicolon. 

**No need for curly brackets or parentheses!** Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses.