// https://doc.rust-lang.org/reference/items/modules.html

mod math {
    type Complex = (f64, f64);
    fn sin(f: f64) -> f64 {
        /* ... */
    }
    fn cos(f: f64) -> f64 {
        /* ... */
    }
    fn tan(f: f64) -> f64 {
        /* ... */
    }
}

#[path = "foo.rs"]
mod c;

mod inline {
    #[path = "other.rs"]
    mod inner;
}
