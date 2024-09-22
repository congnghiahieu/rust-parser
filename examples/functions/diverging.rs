// https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html

#![feature(never_type)]

fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}
