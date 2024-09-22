// https://doc.rust-lang.org/reference/expressions/path-expr.html

fn main() {
    local_var;
    globals::STATIC_VAR;
    unsafe { globals::STATIC_MUT_VAR };
    let some_constructor = Some::<i32>;
    let push_integer = Vec::<i32>::push;
    let slice_reverse = <[i32]>::reverse;
}
