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