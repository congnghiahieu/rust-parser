// https://doc.rust-lang.org/reference/items/use-declarations.html#use-declarations

use std::collections::hash_map::{self, HashMap};

fn foo<T>(_: T) {}
fn bar(map1: HashMap<String, usize>, map2: hash_map::HashMap<String, usize>) {}

fn main() {
    // use declarations can also exist inside of functions
    use std::option::Option::{None, Some};

    // Equivalent to 'foo(vec![std::option::Option::Some(1.0f64),
    // std::option::Option::None]);'
    foo(vec![Some(1.0f64), None]);

    // Both `hash_map` and `HashMap` are in scope.
    let map1 = HashMap::new();
    let map2 = hash_map::HashMap::new();
    bar(map1, map2);
}
