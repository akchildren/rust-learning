# Enums
> Enums allow you to define a type by enumerating its possible variants

### Defining a Enum
Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.

Any IP address can be either a `v4` or a `v6` address, but not both at the same time. 

That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants. 

Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

### Enum Values
We can create instances of each of the two variants of `IpAddrKind` like this:
```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. 

This is useful because now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind:

```rust
fn route(ip_kind: IpAddrKind) {}
```

nd we can call this function with either variant:
```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```

Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is. Given that you just learned about structs in Chapter 5, you might be tempted to tackle this problem with structs.

```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```

### Another example
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

**This enum has four variants with different types:**

* Quit has no data associated with it at all.
* Move has named fields, like a struct does.
* Write includes a single String.
* ChangeColor includes three i32 values.

This would be much cleaner than having multiple structs doing the same thing, here is an example of the long winded way:
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

 > If we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined previously, which is a single type.
