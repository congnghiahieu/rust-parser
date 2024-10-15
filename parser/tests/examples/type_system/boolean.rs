// https://doc.rust-lang.org/reference/types/boolean.html

fn main() {
    // Boolean literals
    let is_true = true;
    let is_false = false;

    // Boolean operations
    let and_result = is_true && is_false;
    let or_result = is_true || is_false;
    let not_result = !is_true;

    // Conditional statements
    if is_true {
        println!("It is true!");
    } else {
        println!("It is false!");
    }

    // Boolean conversions
    let num: u8 = 42;
    let bool_from_num = num != 0;
    let num_from_bool: u8 = is_true as u8;

    // Displaying boolean values
    println!("is_true: {}", is_true);
    println!("is_false: {}", is_false);
    println!("and_result: {}", and_result);
    println!("or_result: {}", or_result);
    println!("not_result: {}", not_result);
    println!("bool_from_num: {}", bool_from_num);
    println!("num_from_bool: {}", num_from_bool);
}
