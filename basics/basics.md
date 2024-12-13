# Basics
Common language understandings

## Variables and immutability
Every variable is immutable by default, keyword mut does change its mutability.

### Constants
Constants are different than variables, because they are always immutable and their values
must be constants.

Their data type are not inferred, so they must be explicitly typed.

### Shadowing
In Rust, you can redeclare a variable, many times as you need it and also explicity change
their data types in declaration.

## Data types
Values are categorized by data types. There are two subsets of them: scalar and compound.

### Scalar
Represents a single value. There are integers, float-pointing numbers, booleans and chars.

#### Integers
Numbers can be signed (with a sign) and unsigned, as if they were written in paper.

Signed numbers data types are followed by the prefix i and unsigned are followed by u.

Their bit lengths are 8, 16, 32, 64 and 128, very common.

#### Floating-point numbers
Rust have two types of them: f32 and f64.

f32 consists into 32 bits and f64, 64 bits.

f32 stands for float and f64, double in most languages.

#### Boolean
Traditionally, represents false and true values.

#### Chars (characters)
They are declared with single quotes and also support unicodes.

#### Mathematical operations
Language supports addition, subtraction, multiplication, division and remainder.

| Operation        | Symbol |
|------------------|--------|
| Addition         | +      |
| Subtraction      | -      |
| Multiplication   | *      |
| Division         | /      |
| Remainder        | %      |

### Compound
"Group multiple values into one type".

### Tuples
Group values into parenthesis and they can differ in types.

Once group is defined, its values length cannot be changed.

All indexes can be accessed using dot (.) operator followed by a selected array position.

### Array
Group values into square brackets, they all must have the same type.

Also length value fixed, once defined.

Indexes are accessed using square brackets operator followed by a selected array position.

Type annotation supports length fixing.

Declaring it with a number separated by a semicolon and number, actually repeats N times.

## Functions
They are "prevalent", most used function is main.

Their names can be defined with lowercase and underscores, snake_case.

Rust doesn't care about their definition orders.

### Parameters
They are "special variables" which deliberately need to be part of a function and contain
a data type.

### Statements and expressions
Statements perform actions and don't return values, expressions do.

### Return functions
You must type a function return with -> operator and a following data type.

Last statement must not be followed by semicolon, so Rust infers it as the return.

## Comments
Lines ignored by compiller, used for code better reading.

## Control Flow
Statements that control whether a block of code executes or not based in conditions.

### if expressions
"if condition is met, run this block of code, if the condition is not met, do not run this
line of code".

Condition must not be followed by parenthesis.

### else if expressions
You can chain multiple else if statements.

### if let expressions
It works like a ternary operator, without ? and : operators, but if and else instead.

### loop statement
It's a while true equivalent, runs a block of code repeatedly till you explicitly break.

### Returning loop statements
You can return a value from a loop statement and then store it in a variable.

### Nested loop statements
You must need disambiguate them with a label, can be a string.

### While loop
Runs a block of code repeatedly whilst a condition is met.

## For loop
You can use a traditional for-in statement to loop through a collection, for example.

You also can use a range in a for-in.