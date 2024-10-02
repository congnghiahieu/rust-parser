// https://doc.rust-lang.org/reference/patterns.html#path-patterns

#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
}

fn main() {
    let color = Color::Red;
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        // Don't need another arm because all variants have been examined
    }
}
