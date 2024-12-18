// https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds

fn f<'a, 'b, 'c, 'd: 'c>(x: &'a i32, mut y: &'b i32, z: &'c i32)
where
    'a: 'b,
{
    y = x; // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0; // &'b &'a i32 is well formed because 'a: 'b
}
