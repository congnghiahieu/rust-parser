// https://doc.rust-lang.org/reference/paths.html#qualified-paths

// Fully qualified paths allow for disambiguating the path for trait implementations and for specifying canonical paths. When used in a type specification, it supports using the type syntax specified below.

struct S;

impl S {
    fn f() {
        println!("S");
    }
}

trait T1 {
    fn f() {
        println!("T1 f");
    }
}

impl T1 for S {}

trait T2 {
    fn f() {
        println!("T2 f");
    }
}

impl T2 for S {}

fn main() {
    S::f(); // Calls the inherent impl.
    <S as T1>::f(); // Calls the T1 trait function.
    <S as T2>::f(); // Calls the T2 trait function.
}
