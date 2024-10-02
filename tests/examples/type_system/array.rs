// https://doc.rust-lang.org/reference/types/array.html

fn main() {
    // Declare an array of integers with a fixed size of 5
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Access individual elements of the array
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Update an element of the array
    let mut names: [String; 3] = [
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
    ];
    names[2] = "David".to_string();

    // Iterate over the elements of the array
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // Get the length of the array
    println!("Array length: {}", numbers.len());
}
