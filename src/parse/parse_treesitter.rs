use std::fs;
use tree_sitter::{Node, Parser};
use tree_sitter_rust;
use tree_sitter_traversal::{traverse_tree, Order};

fn get_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    parser
}

pub fn parse_by_treesitter() {
    let Ok(code) = fs::read_to_string("./examples/special/ast.rs") else {
        panic!("Error reading source code");
    };

    let mut parser = get_parser();
    let Some(tree) = parser.parse(code, None) else {
        panic!("Error parsing source code");
    };

    let root_node = tree.root_node();
    let preorder: Vec<Node<'_>> = traverse_tree(&tree, Order::Pre).collect::<Vec<_>>();
    let postorder: Vec<Node<'_>> = traverse_tree(&tree, Order::Post).collect::<Vec<_>>();

    fs::create_dir_all("./output/treesitter").unwrap();
    fs::write(
        "./output/treesitter/sexp.txt",
        format!(
            "{:#?}: {:#?}",
            root_node.start_position(),
            root_node.end_position()
        ),
    )
    .unwrap();
    fs::write(
        "./output/treesitter/preorder.txt",
        format!("{:#?}", preorder),
    )
    .unwrap();
    fs::write(
        "./output/treesitter/postorder.txt",
        format!("{:#?}", postorder),
    )
    .unwrap();

    walk_tree(root_node, 0, &print_node);
}

fn walk_tree(node: Node, level: usize, callback: &impl Fn(Node, usize) -> ()) {
    if node.child_count() == 0 {
        return;
    }

    callback(node, level);

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            walk_tree(child, level + 2, callback);
        }
    }
}

fn print_node(node: Node, level: usize) {
    for _ in 0..level {
        print!("  ");
    }

    print!(
        "Kind: {:?} Start: {:?} End: {:?}",
        node.kind(),
        node.start_position(),
        node.end_position(),
    );
    // if let Some(prev) = node.prev_sibling() {
    //     print!(" Prev: {:?}", prev.to_sexp());
    // }
    // if let Some(next) = node.next_sibling() {
    //     print!(" Next: {:?}", next.to_sexp());
    // }
    println!();
}
