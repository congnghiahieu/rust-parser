// https://doc.rust-lang.org/reference/items/extern-crates.html

extern crate std; // equivalent to: extern crate std as std;

extern crate std as _; // Underscore Imports

extern crate std as ruststd; // linking to 'std' under another name

extern crate self as good; // linking to the current crate under another name

fn main() {}
