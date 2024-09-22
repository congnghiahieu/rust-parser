// https://doc.rust-lang.org/reference/visibility-and-privacy.html#re-exporting-and-visibility

// Rust allows publicly re-exporting items through a pub use directive. Because this is a public directive, this allows the item to be used in the current module through the rules above. It essentially allows public access into the re-exported item.

// This means that any external crate referencing implementation::api::f would receive a privacy violation, while the path api::f would be allowed.

pub use self::implementation::api;

mod implementation {
    pub mod api {
        pub fn f() {}
    }
}
