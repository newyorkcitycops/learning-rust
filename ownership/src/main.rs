fn main() {
    // String literal
    let s = "Hello World";
    println!("{s}");

    // String
    let mut s2 = String::from("something");
    s2.push_str(" in the way");
    println!("{s2}");

    // Mutable references
    let mut arr = [1, 2, 3];
    println!("Array before changing: {:?}", arr);

    change_last_index(&mut arr);
    
    println!("Array after changing: {:?}", arr);

    // Slices
    let first_word = &s[..5];
    let second_word = &s[6..];

    let second_to_last_items = &arr[1..];

    println!("First word of {s} string: {first_word}");
    println!("Second word of {s} string: {second_word}");

    println!("Second to last items of array: {:?}", second_to_last_items);

}

fn change_last_index(arr: &mut [i32]) {
    arr[arr.len() - 1] = 4;
}
