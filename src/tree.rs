use tree_sitter::{Node, Tree};

/// Pre-order traversal of the syntax tree that triggers @node_fn at each node
/// TODO This could be cleaned up to return an iterator
pub fn traverse<CB>(tree: &Tree, mut node_fn: CB)
where
    CB: FnMut(&Node),
{
    let mut cursor = tree.walk();
    'outer: loop {
        let node = cursor.node();
        node_fn(&node);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preorder_traversal() {
        let text = r"
fn main() {
    let foo = 1;
    let bar = if foo > 2 {
        true
    } else {
        false
    };
}";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_rust::language()).unwrap();
        let tree = parser.parse(&text, None).unwrap();
        let kinds = vec![
            "source_file",
            "function_item",
            "fn",
            "identifier",
            "parameters",
            "(",
            ")",
            "block",
            "{",
            "let_declaration",
            "let",
            "identifier",
            "=",
            "integer_literal",
            ";",
            "let_declaration",
            "let",
            "identifier",
            "=",
            "if_expression",
            "if",
            "binary_expression",
            "identifier",
            ">",
            "integer_literal",
            "block",
            "{",
            "boolean_literal",
            "true",
            "}",
            "else_clause",
            "else",
            "block",
            "{",
            "boolean_literal",
            "false",
            "}",
            ";",
            "}",
        ];
        let mut i = 0;
        traverse(&tree, |node| {
            assert_eq!(kinds[i], node.kind());
            i += 1;
        });
    }
}
