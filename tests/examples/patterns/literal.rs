// https://doc.rust-lang.org/reference/patterns.html#literal-patterns

fn main() {
    for i in -2..5 {
        match i {
            -1 => println!("It's minus one"),
            1 => println!("It's a one"),
            2 | 4 => println!("It's either a two or a four"),
            _ => println!("Matched none of the arms"),
        }
    }
}
