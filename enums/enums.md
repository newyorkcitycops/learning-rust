# Enums
Enumerates possible variants of a value.

## Enum values
Use :: operator for acessing enum variants.

### Enum types
Enums can have multiple values of different types.

An other enum can be variant of an enum, a variant can also be a struct.

Basically a variant can hold multiple values in its "constructor".

## Option
Rust doesn't have nulls, it has Option enum which holds two variants None and Some.

An Option enum is a generic type, if you set its value to None, you must set a type.

Operation between Option and other data types is not valid.

### Unwrap option values
There's multiple suitable methods, as unwrap, ok_or, take and more.

A match statement can also unwrap values, its arms can validate Some and None conditions.

## match control flow
Consists in validating some enum variants and performing some action or returning a value.

It's not limited to enum variants, thought. Any kind of valid pattern can be a condition.

## Catch-all patterns
Every arm must cover all variants possibilities of a value, otherwise it will raise error.

Use underscore pattern (catch-all) as a default arm.

## if let control flow
When you have to match only one condition, if let should do.

if let can use Some and compare to some other value, to perform some action.

if let can be added with else, so it does simplify match control flows with Some and None.

