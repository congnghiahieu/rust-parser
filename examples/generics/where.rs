// https://doc.rust-lang.org/reference/items/generics.html#where-clauses

struct A<T>
where
    T: Iterator,          // Could use A<T: Iterator> instead
    T::Item: Copy,        // Bound on an associated type
    String: PartialEq<T>, // Bound on `String`, using the type parameter
    i32: Default,         // Allowed, but not useful
    f: T, {}

// https://doc.rust-lang.org/stable/rust-by-example/generics/where.html

impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause
impl<A, D> MyTrait<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
