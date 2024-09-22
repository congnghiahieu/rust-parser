// https://doc.rust-lang.org/reference/items/implementations.html#generic-implementations

impl<T> Seq<T> for Vec<T> {
    /* ... */
}
impl Seq<bool> for u32 {
    /* Treat the integer as a sequence of bits */
}
// T constrains by being an argument to GenericTrait.
impl<T> GenericTrait<T> for i32 {
    /* ... */
}

// T constrains by being an argument to GenericStruct
impl<T> Trait for GenericStruct<T> {
    /* ... */
}

// Likewise, N constrains by being an argument to ConstGenericStruct
impl<const N: usize> Trait for ConstGenericStruct<N> {
    /* ... */
}

// T constrains by being in an associated type in a bound for type `U` which is
// itself a generic parameter constraining the trait.
impl<T, U> GenericTrait<U> for u32
where
    U: HasAssocType<Ty = T>,
{
    /* ... */
}

// Like previous, except the type is `(U, isize)`. `U` appears inside the type
// that includes `T`, and is not the type itself.
impl<T, U> GenericStruct<U>
where
    (U, isize): HasAssocType<Ty = T>,
{
    /* ... */
}

// The rest of these are errors, since they have type or const parameters that
// do not constrain.

// T does not constrain since it does not appear at all.
impl<T> Struct {
    /* ... */
}

// N does not constrain for the same reason.
impl<const N: usize> Struct {
    /* ... */
}

// Usage of T inside the implementation does not constrain the impl.
impl<T> Struct {
    fn uses_t(t: &T) { /* ... */
    }
}

// T is used as an associated type in the bounds for U, but U does not constrain.
impl<T, U> Struct
where
    U: HasAssocType<Ty = T>,
{
    /* ... */
}

// T is used in the bounds, but not as an associated type, so it does not constrain.
impl<T, U> GenericTrait<U> for u32 where U: GenericTrait<T> {}

impl<'a> Struct {}

impl<'a> HasAssocType for Struct {
    type Ty = &'a Struct;
}
