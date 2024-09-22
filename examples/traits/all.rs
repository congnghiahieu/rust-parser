// https://doc.rust-lang.org/reference/items/traits.html

// Traits
trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
    type TypeNoDefault;
    fn method_without_default(&self);
    fn method_with_default(&self) {}
}

// Generic Traits
trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, f: F)
    where
        F: Fn(T);
}

// Supertraits
trait Shape {
    fn area(&self) -> f64;
}
trait Circle
where
    Self: Shape,
{
    fn radius(&self) -> f64 {
        // A = pi * r^2
        // so algebraically,
        // r = sqrt(A / pi)
        (self.area() / std::f64::consts::PI).sqrt()
    }
}

// Unsafe traits
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}
