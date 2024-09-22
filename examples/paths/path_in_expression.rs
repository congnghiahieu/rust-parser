// https://doc.rust-lang.org/reference/paths.html#paths-in-expressions

// Paths in expressions allow for paths with generic arguments to be specified. They are used in various places in expressions and patterns.
// The :: token is required before the opening < for generic arguments to avoid ambiguity with the less-than operator. This is colloquially known as “turbofish” syntax.

fn main() {
    (0..10).collect::<Vec<_>>();
    Vec::<u8>::with_capacity(1024);
}
