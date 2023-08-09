# Keywords

Useful terminoligies when speaking in coding lingo to those who are very litteral.

* **Synonymous**: having the same meaning as another word or phrase in the same language
* **Arbitrary**: based on random choice or personal whim, rather than any reason or system.
* **Function Expression**: Function return value/ term.
* **Function Statement**: `instructions` that perform some action and do not return a value.
* **Unsigned Integer**: Possitive Integer (Can only be positive)
* **Signed Integer**: Generic Integer (Can be possitive or negative [Positive max range is lower than unsigned as it needs to catter for negative range])
* **Integer overflow**: When an integer reaches it's max range and is overflowing in value
* **Disambiguate**: remove uncertainty of meaning
* **Loop Label**: Labels/keyword to explain purpose of loop
* **Allocating on the heap**: The `memory allocator` finds an empty spot in the heap that is big enough, marks it as being in use, and returns a `pointer`, which is the address of that location.
* **Heap Pointer**: keeps track the top of `heap` similar to the stack pointer of the stack
* **String Litteral**: Hardcoded string value in text of application.
* **Garbage Collector**: Keeps track of and cleans up memory that isn’t being used anymore (In other languages, NOT Rust)
* **Shallow Copy**: stores the references of objects to the original memory address (stack) <- **This is known as a `move` in rust**
* **Deep Copy**: Stores copy of the objects value
* **Data Race**: Data races are a common problem in multithreaded programming. Data races occur when multiple tasks or threads access a shared resource without sufficient protections, leading to undefined or unpredictable behavior.
* **Dangling pointer**: a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
* **Struct**: A `struct`, or `structure`, is a custom data type that lets you **package** together and name multiple related values that make up a meaningful group. 
* **tuple structs**: `Structs` that have no field names and take a tuple of values
* **unit-like structs**: `Structs` with no fields or value
* **Implementation Block `Impl`**: The `impl` keyword is primarily used to define implementations on types. Inherent implementations are standalone, while trait implementations are used to implement traits for types, or other traits.
* **Associated functions**: Methods definied within a `impl` because they’re associated with the type named after the `impl`.
* **Option<T>**: A enum option that is used instead of a nullable varriable. This is included in the prelude by default
* **match**: The equivalent of a switch statement in PHP
* **if let**: lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest. 
* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules** and use: Let you control the organization, scope, and privacy of paths
* **Paths**: A way of naming an item, such as a struct, function, or module