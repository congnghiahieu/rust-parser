// https://doc.rust-lang.org/reference/statements.html#declaration-statements

// Item declarations
fn outer() {
    let outer_var = true;

    fn inner() { /* outer_var is not in scope here */
    }

    inner();
}

// let statements
fn main() {
    let (mut v, w) = (vec![1, 2, 3], 42); // The bindings may be mut or const
    let Some(t) = v.pop() else {
        // Refutable patterns require an else block
        panic!(); // The else block must diverge
    };
    let [u, v] = [v[0], v[1]] else {
        // This pattern is irrefutable, so the compiler
        // will lint as the else block is redundant.
        panic!();
    };
}
