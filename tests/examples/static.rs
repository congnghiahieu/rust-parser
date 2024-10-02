// https://doc.rust-lang.org/reference/items/static-items.html

// Mutable statics

static mut LEVELS: u32 = 0;

// This violates the idea of no shared state, and this doesn't internally
// protect against races, so this function is `unsafe`
unsafe fn bump_levels_unsafe1() -> u32 {
    let ret = LEVELS;
    LEVELS += 1;
    return ret;
}

// Statics & generics
use std::sync::atomic::{AtomicUsize, Ordering};

trait Tr {
    fn default_impl() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        println!(
            "default_impl: counter was {}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
    }

    fn blanket_impl();
}

struct Ty1 {}
struct Ty2 {}

impl<T> Tr for T {
    fn blanket_impl() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        println!(
            "blanket_impl: counter was {}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
    }
}

fn main() {
    <Ty1 as Tr>::default_impl(); // default_impl: counter was 0
    <Ty2 as Tr>::default_impl(); // default_impl: counter was 1
    <Ty1 as Tr>::blanket_impl(); // blanket_impl: counter was 0
    <Ty2 as Tr>::blanket_impl(); // blanket_impl: counter was 1
}
