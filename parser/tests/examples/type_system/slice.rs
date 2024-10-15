// https://doc.rust-lang.org/reference/types/slice.html

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Creating a slice from an array
    let slice = &numbers[1..4];

    // Accessing elements in the slice
    println!("Slice: {:?}", slice); // Output: [2, 3, 4]
    println!("First element: {}", slice[0]); // Output: 2

    // Modifying elements in the slice
    let mut mutable_slice = &mut numbers[1..4];
    mutable_slice[0] = 10;
    println!("Modified slice: {:?}", mutable_slice); // Output: [10, 3, 4]
}
