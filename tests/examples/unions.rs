// https://doc.rust-lang.org/reference/items/unions.html

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn f(u: MyUnion) {
    unsafe {
        match u {
            MyUnion { f1: 10 } => {
                println!("ten");
            }
            MyUnion { f2 } => {
                println!("{}", f2);
            }
        }
    }
}

fn main() {
    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
}
