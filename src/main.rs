mod ast;
mod cli;
mod utils;

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{self, Path};
use syn::parse_file;
use tree_sitter::{Node, Parser};
use tree_sitter_rust;

fn main() -> std::io::Result<()> {
    let mut parser = get_parser();
    let dir_path = Path::new("./examples");

    let mut entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    entries.iter().for_each(|entry| {
        if entry.is_file() {
            let mut source_file = File::open(entry).unwrap();
            let mut source_code = String::new();
            source_file.read_to_string(&mut source_code);

            let ast = syn::parse_file(&source_code).unwrap();
            println!("{}", entry.display());
            println!("{:#?}", ast);

            // if let Some(tree) = parser.parse(source_code, None) {
            //     let root_node = tree.root_node();

            //     println!("{}", entry.display());
            //     utils::walk_tree(root_node, 0, &utils::print_node);
            //     println!();
            // };
        }
    });

    Ok(())
}

fn get_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::language())
        .expect("Error loading Rust grammar");
    parser
}
