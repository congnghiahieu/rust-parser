use std::rc::Rc;

// https://doc.rust-lang.org/reference/types/pointer.html

// Shared references (&)
fn shared_references() {
    let x = 5;
    let y = &x; // create a shared reference to x

    println!("Value of x: {}", x);
    println!("Value of y: {}", *y); // dereference y to access the value of x
}

// Mutable references (&mut)
fn mutable_references() {
    let mut x = 5;
    let y = &mut x; // create a mutable reference to x

    *y += 1; // modify the value of x through y

    println!("Value of x: {}", x);
    println!("Value of y: {}", *y);
}

fn raw_pointers() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let x = 5;
    let y = &x as *const i32; // create a raw pointer to x

    unsafe {
        println!("Value of x: {}", x);
        println!("Value of y: {}", *y); // dereference y to access the value of x
    }
}

// Smart Pointers

fn smart_pointers() {
    let x = Rc::new(5); // create a reference-counted smart pointer to 5
    let y = x.clone(); // increase the reference count

    println!("Value of x: {}", *x);
    println!("Value of y: {}", *y);
}
