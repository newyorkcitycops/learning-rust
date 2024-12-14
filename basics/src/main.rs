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

    // Float-pointing numbers
    let float: f32 = 2.0;
    let double: f64 = 2.0000000000000001;
    println!("Float: {float}");
    println!("Double: {double}");

    // Boolean
    let is_even = 0 % 2;
    println!("Zero: {is_even}");

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

    // Functions
    let bits = square(32);
    println!("{bits} bits");

    // Control flow
    let is_palindrome = palindrome(313);
    println!("313 is palindrome: {is_palindrome}");
}

fn square(num: i32) -> i32 {
    num * 2
}

fn palindrome(num: i32) -> bool {
    let original = num;
    let mut reversed = 0;
    let mut remainder;
    let mut n = num;

    while n != 0 {
        remainder = n % 10;
        reversed = reversed * 10 + remainder;
        n /= 10
    }

    original == reversed
}