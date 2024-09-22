// https://doc.rust-lang.org/reference/patterns.html#tuple-patterns

fn main() {
    let pair = (10, "ten");
    let (a, b) = pair;

    assert_eq!(a, 10);
    assert_eq!(b, "ten");
}
