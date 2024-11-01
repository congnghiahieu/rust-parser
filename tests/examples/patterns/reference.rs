// https://doc.rust-lang.org/reference/patterns.html#reference-patterns

fn main() {
    let int_reference = &3;

    let a = match *int_reference {
        0 => "zero",
        _ => "some",
    };
    let b = match int_reference {
        &0 => "zero",
        _ => "some",
    };

    assert_eq!(a, b);
}
