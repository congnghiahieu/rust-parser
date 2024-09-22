// https://doc.rust-lang.org/reference/items/generics.html

fn foo<'a, T>() {}
trait A<U> {}
struct Ref<'a, T>
where
    T: 'a,
{
    r: &'a T,
}
struct InnerArray<T, const N: usize>([T; N]);
struct EitherOrderWorks<const N: bool, U>(U);
