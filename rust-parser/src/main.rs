mod cli;
mod parse;

use crate::cli::get_args;
use crate::parse::{parse_by_syn, parse_by_treesitter};

fn main() {
    let args = get_args();
    parse_by_syn(&args);
}
