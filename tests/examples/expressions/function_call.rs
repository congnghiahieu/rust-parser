// https://doc.rust-lang.org/reference/expressions/call-expr.html

// Call expressions
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn call_expresions() {
    let three: i32 = add(1i32, 2i32);
    let name: &'static str = (|| "Rust")();
}

// Disambiguating Function Calls
struct Foo;
struct Bar;

trait Pretty {
    fn print(&self);
}

trait Ugly {
    fn print(&self);
}

impl Pretty for Foo {
    fn print(&self) {}
}

impl Pretty for Bar {
    fn print(&self) {}
}
impl Ugly for Bar {
    fn print(&self) {}
}

fn main() {
    let f = Foo;
    let b = Bar;

    // we can do this because we only have one item called `print` for `Foo`s
    f.print();
    // more explicit, and, in the case of `Foo`, not necessary
    Foo::print(&f);
    // if you're not into the whole brevity thing
    <Foo as Pretty>::print(&f);

    // b.print(); // Error: multiple 'print' found
    // Bar::print(&b); // Still an error: multiple `print` found

    // necessary because of in-scope items defining `print`
    <Bar as Pretty>::print(&b);
}
