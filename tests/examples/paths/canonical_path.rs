// https://doc.rust-lang.org/reference/paths.html#canonical-paths

// Items defined in a module or implementation have a canonical path that corresponds to where within its crate it is defined. All other paths to these items are aliases. The canonical path is defined as a path prefix appended by the path segment the item itself defines.
// Implementations and use declarations do not have canonical paths, although the items that implementations define do have them. Items defined in block expressions do not have canonical paths. Items defined in a module that does not have a canonical path do not have a canonical path. Associated items defined in an implementation that refers to an item without a canonical path, e.g. as the implementing type, the trait being implemented, a type parameter or bound on a type parameter, do not have canonical paths.

// Comments show the canonical path of the item.

mod a {
    // crate::a
    pub struct Struct; // crate::a::Struct

    pub trait Trait {
        // crate::a::Trait
        fn f(&self); // crate::a::Trait::f
    }

    impl Trait for Struct {
        fn f(&self) {} // <crate::a::Struct as crate::a::Trait>::f
    }

    impl Struct {
        fn g(&self) {} // <crate::a::Struct>::g
    }
}

mod without {
    // crate::without
    fn canonicals() {
        // crate::without::canonicals
        struct OtherStruct; // None

        trait OtherTrait {
            // None
            fn g(&self); // None
        }

        impl OtherTrait for OtherStruct {
            fn g(&self) {} // None
        }

        impl OtherTrait for crate::a::Struct {
            fn g(&self) {} // None
        }

        impl crate::a::Trait for OtherStruct {
            fn f(&self) {} // None
        }
    }
}
