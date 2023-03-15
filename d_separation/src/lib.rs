use std::cell::{Cell, RefCell};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    children: Vec<RefCell<Node>>,
}

struct Graph {
    nodes: Vec<Cell<Node>>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: vec![] }
    }

    fn add_node(&mut self, name: &str) -> RefCell<Node> {
        let node = Node {
            name: name.to_string(),
            children: vec![],
        };
        self.nodes.push(Cell::new(node.clone()));
        RefCell::new(node)
    }

    fn add_edge(&mut self, parent: &RefCell<Node>, child: &RefCell<Node>) {
        parent.borrow_mut().children.push(child.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
