# Structs
Custom data type that group variables in a meaningful and similar way as OOP.

## Defining structs
A struct consist into a name and fields, and you should associate fields with the name.

Structs cannot have reference fields, its entire need to be mutable, instead.

e.g: A user struct could contain name and email fields.

### Field shorthand
You can define structs with variables/parameters that hold same names as fields.

### Creating structs from other structs instances
You can pass fields from one struct to another, but this is not recommended.

As a better option, you can actually use the .. operator, so you update only the changed
fields and keep other fields values.

### Tuple structs
Structs can be defined as tuples, as they are very similar.

A tuple struct is accesed with dot (.) operator and a selected integer position.

### Unit structs
They are useful for applying traits, they don't have any fields.

## Using structs
Whenever you see a meaningful variables group, create a struct/tuple.

### Derived traits
Derive annotation for structs is commonly used with Debug trait.

## Struct methods
Essentially a function that struct holds its implementation and assign an instance to it.

### Defining methods
You must use impl block followed by struct's name and therefore create an inner fn block.

### Multiple parameters methods
It is possible to hold a self instance and other same type instance, as parameters.

### Associated functions
They don't hold struct instance, but return Self (new instance) as a "constructor method".

They are used with :: operator, insted of dot (.) operator.
