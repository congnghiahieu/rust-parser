// https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern

fn main() {
    let (a, _) = (10, x); // the x is always matched by _

    // ignore a function/closure param
    let real_part = |a: f64, _: f64| a;

    // ignore a field from a struct
    let RGBA {
        r: red,
        g: green,
        b: blue,
        a: _,
    } = color;

    // accept any Some, with any value
    if let Some(_) = x {}
}
