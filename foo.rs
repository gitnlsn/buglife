struct Node {
    value: usize,
    next: Option<Node>
}

impl Node {

    fn new(val: usize) -> Node {
        Node {
            value: val,
            next: None,
        }
    }
    
    fn append_to(&mut self, node: Node) {
        self.next = Some(node);
    }
    
    fn total(&mut self) {
        match self.next {
            Some(node) => return self.value + node.total,
            None => return self.value,
        }
    }
}

#[test]
fn test_recursion() {
    
}

