// https://doc.rust-lang.org/reference/expressions/struct-expr.html

// Tuple struct expression
fn tuple_struct() {
    struct Position(i32, i32, i32);
    Position(0, 0, 0); // Typical way of creating a tuple struct.
    let c = Position; // `c` is a function that takes 3 arguments.
    let pos = c(8, 6, 7); // Creates a `Position` value.
}

// Unit struct expression
fn unit_struct() {
    struct Gamma;
    let a = Gamma; // Gamma unit value.
    let b = Gamma {}; // Exact same value as `a`.
}

// Struct field init shorthand
fn struct_field_init_shorthand() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let x = 1;
    let y = 2;
    let z = 3;
    let p = Point { x, y, z }; // Equivalent to `Point { x: x, y: y, z: z }`.
}

// Functional update syntax
fn functional_update_syntax() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let p = Point { x: 1, y: 2, z: 3 };
    let p2 = Point { x: 4, ..p }; // Equivalent to `Point { x: 4, y: 2, z: 3 }`.

    struct Color(u8, u8, u8);
    let c1 = Color(0, 0, 0); // Typical way of creating a tuple struct.
    let c2 = Color {
        0: 255,
        1: 127,
        2: 0,
    }; // Specifying fields by index.
    let c3 = Color { 1: 0, ..c2 }; // Fill out all other fields using a base struct.
}
