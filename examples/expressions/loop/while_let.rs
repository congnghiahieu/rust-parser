// https://doc.rust-lang.org/stable/rust-by-example/flow_control/while_let.html

fn main() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}

fn have_pattern() {
    let mut vals = vec![2, 3, 1, 2, 2];
    while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
        // Prints 2, 2, then 1
        println!("{}", v);
    }
}
