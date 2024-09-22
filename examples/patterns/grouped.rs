// https://doc.rust-lang.org/reference/patterns.html#grouped-patterns

fn main() {
    let int_reference = &3;
    match int_reference {
        &(0..=5) => (),
        _ => (),
    }
}
