// https://doc.rust-lang.org/reference/expressions/block-expr.html

fn test() {
    let _: () = {
        fn_call();
    };

    let five: i32 = {
        fn_call();
        5
    };

    assert_eq!(5, five);
}

// async blocks

// const blocks
fn foo<T>() -> usize {
    // If this code ever gets executed, then the assertion has definitely
    // been evaluated at compile-time.
    const {
        assert!(std::mem::size_of::<T>() > 0);
    }
    // Here we can have unsafe code relying on the type being non-zero-sized.
    /* ... */
    42
}

// unsafe blocks
fn main() {
    unsafe {
        let b = [13u8, 17u8];
        let a = &b[0] as *const u8;
        assert_eq!(*a, 13);
        assert_eq!(*a.offset(1), 17);
    }

    let a = unsafe { an_unsafe_fn() };
}
