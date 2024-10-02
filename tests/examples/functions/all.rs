// https://doc.rust-lang.org/reference/items/functions.html

use std::fmt::Debug;

// For example, this is a simple function:
fn answer_to_life_the_universe_and_everything() -> i32 {
    return 42;
}

// Function parameters
fn first((value, _): (i32, i32)) -> i32 {
    // Function body
    let (value, _) = argument_0;
    return { value };
}

// Generic functions
fn foo<A, B>(x: A, y: B) {}

fn bar<T>(x: &[T])
where
    T: Debug,
{
}

// Extern function qualifier
fn main() {
    // Declares a function with the "C" ABI
    extern "C" fn new_i32() -> i32 {
        0
    }

    // Declares a function with the "stdcall" ABI
    extern "stdcall" fn new_i32_stdcall() -> i32 {
        0
    }

    extern "ABI" {
        fn foo(); /* no body */
    }
    unsafe { foo() }
}

// Const functions
const fn second((value, _): (i32, i32)) -> i32 {
    value
}

// Async functions
async unsafe fn unsafe_example(x: *const i32) -> i32 {
    *x
}
async fn safe_example() {
    // An `unsafe` block is required to invoke the function initially:
    let p = 22;
    let future = unsafe { unsafe_example(&p) };

    // But no `unsafe` block required here. This will
    // read the value of `p`:
    let q = future.await;
}

// Attributes on functions
fn documented() {
    #![doc = "Example"]
}

// Attributes on function parameters
fn len(#[cfg(windows)] slice: &[u16], #[cfg(not(windows))] slice: &[u8]) -> usize {
    slice.len()
}
