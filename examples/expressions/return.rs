// https://doc.rust-lang.org/reference/expressions/return-expr.html

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }

    return b;
}
