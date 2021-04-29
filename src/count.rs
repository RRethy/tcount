use tree_sitter::Tree;

pub fn count_tokens(tree: &Tree) -> u64 {
    let mut ntokens = 0;
    let mut cursor = tree.walk();

    // preoder traversal of the concrete syntax tree
    'outer: loop {
        let node = cursor.node();
        // TODO handle comments via cli maybe
        if node.child_count() == 0 && !node.is_extra() && !node.is_missing() {
            // TODO what if this is not a terminal node
            ntokens += 1;
        }
        if !cursor.goto_first_child() && !cursor.goto_next_sibling() {
            'inner: loop {
                if !cursor.goto_parent() {
                    break 'outer;
                }
                if cursor.goto_next_sibling() {
                    break 'inner;
                }
            }
        }
    }
    ntokens
}
