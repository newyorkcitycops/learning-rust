fn main() {
    // Immutability and mutability
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    const VACATION_DAYS: i32 = 30;
    println!("You have {VACATION_DAYS} remaining vacation days");

    // Shadowing
    let y = 42;
    let y: char = '🦀';
    println!("{y}");

    // Integers
    let signed: u32 = 10;
    let unsigned: i32 = -10;
    println!("Signed: {signed}");
    println!("Unsigned: {unsigned}");

    // Floating-point numbers
    let float: f32 = 2.0;
    let double: f64 = 2.0000000000000001;
    println!("Float: {float}");
    println!("Double: {double}");

    // Boolean
    let is_even = 0 % 2;
    println!("Even: {is_even}");

    // Char
    let dad ='👨';
    // Zero width joiners emojis can't be chars in Rust
    // let family = '👨‍👩‍👧‍👦';
    println!("{}", dad);

    // Tuple
    let row: (i32, &str, char) = (1, "John Smith", 'M');
    println!("{:?}", row);

    // Array
    let numbers = [3; 5];
    println!("{:?}", numbers);
    println!("Number: {0}", numbers[1]);
}