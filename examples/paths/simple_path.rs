// https://doc.rust-lang.org/reference/paths.html#simple-paths

use std::io::{self, Write};
mod m {
    #[clippy::cyclomatic_complexity = "0"]
    pub(super) fn f1() {}
}
