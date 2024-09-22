// https://doc.rust-lang.org/reference/attributes.html#meta-item-attribute-syntax

#[derive(Serialize)]
struct Foo {
    #[doc = include_str!("x.md")]
    x: u32,
}

#[macro_attr1] // expanded first
#[doc = mac!()] // `mac!` is expanded fourth.
#[macro_attr2] // expanded second
#[derive(MacroDerive1, MacroDerive2)] // expanded third
fn foo() {}
