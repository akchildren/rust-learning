# Methods
`Methods` are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. 

> Unlike functions, methods are defined within the context of a `struct` (or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

### Defining Methods
Example 

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

### Methods: Impl (Implementation method)

**To define the function within the context of Rectangle, we start an `impl` (implementation) block for Rectangle.**

>Everything within this impl block will be associated with the Rectangle type. 

Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body. 

In main, where we called the `area` function and passed `rect1` as an argument, we can instead use method syntax to call the `area` method on our Rectangle instance. 

The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.

### Methods: Self 

In the signature for area, we use &`self` instead of rectangle: &Rectangle. 

The `&self` is actually short for self: `&Self`. Within an impl block, the type Self is an alias for the type that the impl block is for. 

> Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot. 

Note that we still need to use the `&` in front of the `self` shorthand to indicate that this method **borrows** the Self instance, just as we did in rectangle: &Rectangle. 

Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

### Methods: Reasons for using methods over functions
The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of `self` in every method’s signature, is for **organization**. 

We’ve put all the things we can do with an instance of a type in one `impl` block rather than making future users of our code search for capabilities of `Rectangle` in various places in the library we provide.

### Methods: Field names and method names can be the same (if needed)

Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle that is also named width:

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

Here, we’re choosing to make the `width` method return `true` if the value in the instance’s `width` field is greater than `0` and `false` if the value is `0`: we can use a field within a method of the same name for any purpose. 

In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. When we don’t use parentheses, Rust knows we mean the field width. (Similar to laravel models)

### Methods: Getters
> Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. 

Methods like this are called `getters`, and Rust does not implement them automatically for struct fields as some other languages do. 

> `Getters` are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API.

### Methods: More Parameters
Let’s practice using methods by implementing a second method on the `Rectangle` struct. This time we want an instance of `Rectangle` to take another instance of `Rectangle` and return `true` if the second `Rectangle` can fit completely within `self` (the first `Rectangle`); otherwise, it should return `false`. 

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

We know we want to define a method, so it will be within the `impl Rectangle `block. 

The method name will be `can_hold`, and it will take an immutable borrow of another Rectangle as a parameter. 

We can tell what the type of the parameter will be by looking at the code that calls the method: `rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to `rect2`, an instance of `Rectangle`. 

This makes sense because we only need to read `rect2` (rather than write, which would mean we’d need a mutable borrow), and we want main to retain ownership of rect2 so we can use it again after calling the can_hold method.

### Associated Functions
All functions defined within an `impl` block are called `associated functions` because they’re associated with the type named after the `impl`. 

We can define associated functions that don’t have `self` as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. 

We’ve already used one function like this: the String::from function that’s defined on the String type.

`Associated functions` that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called `new`, but `new` isn’t a special name and isn’t built into the language. (Like new Model in laravel)

> For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

The `Self` keywords in the return type and in the body of the function are aliases for the type that appears after the `impl` keyword, which in this case is `Rectangle`.

### Multiple impl Blocks
Each struct is allowed to have multiple impl blocks.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

There’s no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax. 