// https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants

struct Struct;
struct OtherStruct;

trait ConstantId {
    const ID: i32;
}

trait ConstantIdDefault {
    const ID: i32 = 1;
}

impl ConstantId for Struct {
    const ID: i32 = 2;
}

impl ConstantIdDefault for OtherStruct {
    const ID: i32 = 5;
}

fn main() {
    assert_eq!(1, Struct::ID);
    assert_eq!(5, OtherStruct::ID);
}
