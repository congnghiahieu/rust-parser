// https://doc.rust-lang.org/reference/items/external-blocks.html

unsafe extern "C" {
    fn foo(...);
    fn bar(x: i32, ...);
    fn with_name(format: *const u8, args: ...);
}
fn main() {}
