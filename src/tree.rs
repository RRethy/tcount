use tree_sitter::{Node, Tree, TreeCursor};

pub struct TreeIterator<'a> {
    cursor: TreeCursor<'a>,
    next: Option<Node<'a>>,
}

impl<'a> TreeIterator<'a> {
    pub fn new(tree: &'a Tree) -> Self {
        let cursor = tree.walk();
        Self {
            next: Some(cursor.node()),
            cursor,
        }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            let cursor = &mut self.cursor;
            // preoder traverse to find next node
            self.next = if !cursor.goto_first_child() && !cursor.goto_next_sibling() {
                // look for a parent with a sibling that we have yet to visit
                loop {
                    if !cursor.goto_parent() {
                        // we are back at the root of the syntax tree and preorder traversal is
                        // done
                        break None;
                    }
                    if cursor.goto_next_sibling() {
                        break Some(cursor.node());
                    }
                }
            } else {
                Some(cursor.node())
            };
            Some(next)
        } else {
            None
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
        let tree_iter = TreeIterator::new(&tree);
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
        assert_eq!(
            kinds,
            tree_iter.map(|node| node.kind()).collect::<Vec<&str>>()
        );
    }
}
