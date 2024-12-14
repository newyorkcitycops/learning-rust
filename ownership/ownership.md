# Ownership
Consists in rules to achieve a better memory management, as Rust don't have a GC.

## Stack and heap
Stack is organized in a last in, first out manner.

Heap allocates an item whenever an empty spot is met, and to find it you can use pointers.

## Rules
All values are owned, a value can have one owner and whenever they get out of scope, they
get dropped.

## String
It's a complex data type, it's a "collection" of chars.

There is actually two subtypes of strings: str and String.

str is length-fixed whether String isn't.

## Memory and allocation
Every value allocated in memory is dropped whenever they go out of scope.

### Clone
There's a way of deeply copying values in Rust, clone method.

Shallow copies can be done with values like integers.

## References
They are different from pointers, references always point to valid values.

### Mutable references
You can move value from mutable variables using mutable references.

You can't have two mutable references of the same value, nor one mutable and other not.

### Dangling references
Rust prevents references that points to an invalid value.

## Slices
They let you slice parts of a compound value.

### String slices
If you have a string and two slices of that string, they would point to the same string.

### String literals
They are immutable, because they point to a specific point of the binary string.

&str is an immutable reference.

### Array slices
Same as strings, they do point to specific parts of an array.
