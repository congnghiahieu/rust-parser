// https://doc.rust-lang.org/reference/types/never.html

fn foo() -> ! {
    panic!("This call never returns.");
}

extern "C" {
    pub fn no_return_extern_func() -> !;
}
