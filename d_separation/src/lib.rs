use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    name: String,
    edges: Vec<String>,
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: String) -> String {
        let node = Node {
            name: name.clone(),
            edges: Vec::new(),
        };
        self.nodes.insert(name.clone(), node);
        name
    }

    pub fn add_edge(&mut self, from: &String, to: &String) {
        let from_node = self.nodes.get_mut(from).unwrap();
        from_node.edges.push(to.clone());
    }

    pub fn get_children(&self, node_index: &String) -> Vec<String> {
        let node = self.nodes.get(node_index).unwrap();
        node.edges.clone()
    }
}

pub fn add_parents(d_sep_graph: &mut Graph, base_graph: &Graph) {
    let mut changed = true;
    while changed {
        changed = false;
        for possible_parent in base_graph.nodes.keys() {
            let possible_parent_children = base_graph.get_children(possible_parent);
            let d_sep_node_keys: Vec<String> = d_sep_graph.nodes.keys().cloned().collect();

            for node in d_sep_node_keys.iter() {
                if possible_parent_children.contains(node) {
                    if !d_sep_graph.nodes.contains_key(possible_parent) {
                        d_sep_graph.add_node(possible_parent.clone());
                    }
                    if !d_sep_graph.nodes[possible_parent].edges.contains(node) {
                        d_sep_graph.add_edge(possible_parent, node);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_parents_test() {
        let mut base_graph = Graph::new();
        base_graph.add_node("A".to_string());
        base_graph.add_node("B".to_string());
        base_graph.add_node("C".to_string());

        base_graph.add_edge(&"A".to_string(), &"B".to_string());
        base_graph.add_edge(&"A".to_string(), &"C".to_string());

        let mut d_sep_graph = Graph::new();
        d_sep_graph.add_node("B".to_string());
        d_sep_graph.add_node("C".to_string());

        add_parents(&mut d_sep_graph, &base_graph);

        assert_eq!(d_sep_graph.nodes.len(), 3);
    }
}
