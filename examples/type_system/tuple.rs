// https://doc.rust-lang.org/reference/types/tuple.html

fn main() {
    // Tuple with two elements
    let tuple1 = (1, "hello");

    // Accessing tuple elements
    let first_element = tuple1.0;
    let second_element = tuple1.1;

    println!("First element: {}", first_element);
    println!("Second element: {}", second_element);

    // Tuple with three elements
    let tuple2 = (3.14, true, 'a');

    // Destructuring tuple
    let (pi, is_true, letter) = tuple2;

    println!("pi: {}", pi);
    println!("is_true: {}", is_true);
    println!("letter: {}", letter);
}
