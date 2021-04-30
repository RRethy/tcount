use tree_sitter::Tree;

pub fn count(tree: &Tree, counters: &mut Vec<Box<dyn Counter>>) {
    let mut cursor = tree.walk();

    // preoder traversal of the syntax tree
    'outer: loop {
        let node = cursor.node();
        counters
            .iter_mut()
            .for_each(|counter| counter.report(&node));
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

pub trait Counter {
    fn report(&mut self, node: &tree_sitter::Node);
    fn get_count(&self) -> u64;
    fn reset(&mut self);
}

pub struct KindCounter {
    kind: String,
    n: u64,
}

impl KindCounter {
    pub fn new(kind: String) -> Self {
        KindCounter { kind, n: 0 }
    }
}

impl Counter for KindCounter {
    fn report(&mut self, node: &tree_sitter::Node) {
        if self.kind == node.kind() && !node.is_missing() {
            self.n += 1;
        }
    }
    fn get_count(&self) -> u64 {
        self.n
    }
    fn reset(&mut self) {
        self.n = 0;
    }
}

pub struct TokenCounter {
    n: u64,
}

impl TokenCounter {
    pub fn new() -> Self {
        TokenCounter { n: 0 }
    }
}

impl Counter for TokenCounter {
    fn report(&mut self, node: &tree_sitter::Node) {
        if node.child_count() == 0 && !node.is_extra() && !node.is_missing() {
            self.n += 1;
        }
    }
    fn get_count(&self) -> u64 {
        self.n
    }
    fn reset(&mut self) {
        self.n = 0;
    }
}
