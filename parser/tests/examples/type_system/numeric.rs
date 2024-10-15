// https://doc.rust-lang.org/reference/types/numeric.html

fn main() {
    // Integer types
    let signed_integer: i8 = 42;
    let unsigned_integer: u16 = 123;

    // Floating-point types
    let float: f32 = 3.14;
    let double: f64 = 2.71828;

    // Numeric operations
    let sum = signed_integer + unsigned_integer;
    let difference = float - double;
    let product = signed_integer * unsigned_integer;
    let quotient = float / double;
    let remainder = unsigned_integer % signed_integer;

    // Displaying values
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}
