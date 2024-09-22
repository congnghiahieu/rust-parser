// https://doc.rust-lang.org/reference/expressions/field-expr.html

// Field access expressions
fn field_access() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };
    let x = p.x;
    let y = p.y;
}

// Automatic dereferencing

// Borrowing
fn borrowing() {
    struct A {
        f1: String,
        f2: String,
        f3: String,
    }
    let mut x: A;
    let a: &mut String = &mut x.f1; // x.f1 borrowed mutably
    let b: &String = &x.f2; // x.f2 borrowed immutably
    let c: &String = &x.f2; // Can borrow again
    let d: String = x.f3; // Move out of x.f3
}
