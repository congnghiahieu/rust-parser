// https://doc.rust-lang.org/reference/unsafety.html

/*
Unsafe operations are those that can potentially violate the memory-safety guarantees of Rust’s static semantics.

The following language level features cannot be used in the safe subset of Rust:

- Dereferencing a raw pointer.
- Reading or writing a mutable or external static variable.
- Accessing a field of a union, other than to assign to it.
- Calling an unsafe function (including an intrinsic or foreign function).
- Implementing an unsafe trait.
*/

// https://doc.rust-lang.org/reference/unsafety.html

/*
Unsafe operations are those that can potentially violate the memory-safety guarantees of Rust’s static semantics.

The following language level features cannot be used in the safe subset of Rust:

- Dereferencing a raw pointer.
- Reading or writing a mutable or external static variable.
- Accessing a field of a union, other than to assign to it.
- Calling an unsafe function (including an intrinsic or foreign function).
- Implementing an unsafe trait.
*/

#![allow(unused)]

// Dereferencing a raw pointer
fn dereference_raw_pointer() {
    let x: i32 = 42;
    let r: *const i32 = &x;
    unsafe {
        println!("Dereferenced raw pointer: {}", *r);
    }
}

// Reading or writing a mutable or external static variable
static mut COUNTER: i32 = 0;

fn modify_static_variable() {
    unsafe {
        COUNTER += 1;
        println!("Modified static variable: {}", COUNTER);
    }
}

// Accessing a field of a union, other than to assign to it
union MyUnion {
    i: i32,
    f: f32,
}

fn access_union_field() {
    let u = MyUnion { i: 42 };
    unsafe {
        println!("Accessed union field: {}", u.i);
    }
}

// Calling an unsafe function (including an intrinsic or foreign function)
unsafe fn unsafe_function() {
    println!("Called unsafe function");
}

fn call_unsafe_function() {
    unsafe {
        unsafe_function();
    }
}

// Implementing an unsafe trait
pub unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

struct MyStruct;

unsafe impl UnsafeTrait for MyStruct {
    fn unsafe_method(&self) {
        println!("Called unsafe method on unsafe trait");
    }
}

fn call_unsafe_trait_method() {
    let s = MyStruct;
    unsafe {
        s.unsafe_method();
    }
}

fn main() {}
