// https://doc.rust-lang.org/reference/expressions/grouped-expr.html

fn main() {
    let x: i32 = 2 + 3 * 4; // not parenthesized
    let y: i32 = (2 + 3) * 4; // parenthesized
    assert_eq!(x, 14);
    assert_eq!(y, 20);

    assert_eq!(a.f(), "The method f");
    assert_eq!((a.f)(), "The field f");
}
