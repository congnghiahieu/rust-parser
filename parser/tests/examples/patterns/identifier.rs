// https://doc.rust-lang.org/reference/patterns.html#identifier-patterns

fn main() {
    let x = 2;
    match x {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    let a = Some(1);
    match a {
        None => (),
        Some(value) => (),
    }
    match a {
        None => (),
        Some(ref value) => (),
    }
}
