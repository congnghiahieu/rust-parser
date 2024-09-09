use tree_sitter::Node;

pub fn walk_tree(node: Node, level: usize, callback: &impl Fn(Node, usize) -> ()) {
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

pub fn print_node(node: Node, level: usize) {
    for _ in 0..level {
        print!("  ");
    }

    print!(
        "Kind: {:?} Start: {:?} End: {:?} Range: {:?}",
        node.kind(),
        node.start_byte(),
        node.end_byte(),
        node.byte_range(),
    );
    if let Some(prev) = node.prev_sibling() {
        print!(" Prev: {:?}", prev.to_string());
    }
    if let Some(next) = node.prev_sibling() {
        print!(" Next: {:?}", next.to_string());
    }
    println!();
}
