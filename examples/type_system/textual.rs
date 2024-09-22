// https://doc.rust-lang.org/reference/types/textual.html

fn main() {
    let my_string = String::from("Hello, world!");
    println!("{}", my_string);

    let my_str: &str = &my_string;
    println!("{}", my_str);

    let my_string_length = my_string.len();
    println!("Length of my_string: {}", my_string_length);

    let my_string_bytes = my_string.as_bytes();
    println!("Bytes of my_string: {:?}", my_string_bytes);

    let my_string_chars: Vec<char> = my_string.chars().collect();
    println!("Chars of my_string: {:?}", my_string_chars);
}
