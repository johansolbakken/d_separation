use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    name: String,
    edges: HashSet<String>,
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
            edges: HashSet::new(),
        };
        self.nodes.insert(name.clone(), node);
        name
    }

    pub fn add_edge(&mut self, from: &String, to: &String) {
        let from_node = self.nodes.get_mut(from).unwrap();
        from_node.edges.insert(to.clone());
    }

    pub fn get_children(&self, node_index: &String) -> HashSet<String> {
        let node = self.nodes.get(node_index).unwrap();
        node.edges.clone()
    }

    pub fn is_connected(&self, node1: &String, node2: &String) -> bool {
        let node1_children = self.get_children(node1);
        let node2_children = self.get_children(node2);

        node1_children.contains(node2) || node2_children.contains(node1)
    }
}

fn add_parents(d_sep_graph: &mut Graph, base_graph: &Graph) {
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

fn moralize(graph: &mut Graph) {
    let mut new_edges: Vec<(String, String)> = vec![];

    for node1 in graph.nodes.keys() {
        for node2 in graph.nodes.keys() {
            if node1 != node2 {
                for node_1_child in graph.get_children(node1).iter() {
                    if graph.get_children(&node2).contains(node_1_child) {
                        new_edges.push((node1.clone(), node2.clone()));
                    }
                }
            }
        }
    }

    for (node1, node2) in new_edges {
        graph.add_edge(&node1, &node2);
    }
}

fn double_edge(graph: &mut Graph) {
    let mut new_edges: Vec<(String, String)> = vec![];

    for node1 in graph.nodes.keys() {
        for node2 in graph.nodes.keys() {
            if node1 != node2 {
                if graph.is_connected(node1, node2) {
                    new_edges.push((node1.clone(), node2.clone()));
                }
            }
        }
    }

    for (node1, node2) in new_edges {
        graph.add_edge(&node1, &node2);
    }
}

pub fn find_path(
    graph: &Graph,
    node: &String,
    first: &String,
    target: &String,
    seen: &HashMap<String, bool>,
    marked: &Vec<String>,
) -> bool {
    if node == target {
        return true;
    }

    if marked.contains(node) && node != first {
        return false;
    }

    let mut seen_this = seen.clone();
    seen_this.insert(node.clone(), true);

    for child in graph.get_children(node).iter() {
        println!("{:#?}", seen);
        println!("{:#?}", graph);
        if !seen[child] {
            if find_path(graph, child, first, target, &seen_this, marked) {
                return true;
            }
        }
    }

    return false;
}

pub fn is_d_separated(graph: &Graph, first: &String, second: &String, deps: &Vec<String>) -> bool {
    let mut d_sep_graph = Graph::new();
    d_sep_graph.add_node(first.clone());
    d_sep_graph.add_node(second.clone());
    for dep in deps {
        d_sep_graph.add_node(dep.clone());
    }

    add_parents(&mut d_sep_graph, graph);
    moralize(&mut d_sep_graph);
    double_edge(&mut d_sep_graph);

    let mut marked = deps.clone();
    marked.push(first.clone());
    marked.push(second.clone());
    let mut seen: HashMap<String, bool> = HashMap::new();
    for node in d_sep_graph.nodes.keys() {
        seen.insert(node.clone(), false);
    }

    find_path(&d_sep_graph, first, first, second, &seen, &marked)
}

#[cfg(test)]
mod tests {
    use std::vec;

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

    #[test]
    fn moralize_test() {
        let mut base_graph = Graph::new();
        base_graph.add_node("A".to_string());
        base_graph.add_node("B".to_string());
        base_graph.add_node("C".to_string());

        base_graph.add_edge(&"A".to_string(), &"B".to_string());
        base_graph.add_edge(&"C".to_string(), &"B".to_string());

        let mut d_sep_graph = Graph::new();
        d_sep_graph.add_node("B".to_string());
        d_sep_graph.add_node("C".to_string());

        add_parents(&mut d_sep_graph, &base_graph);
        moralize(&mut d_sep_graph);

        assert!(d_sep_graph.nodes["A"].edges.contains("B"));
        assert!(d_sep_graph.nodes["A"].edges.contains("C"));

        assert!(d_sep_graph.nodes["C"].edges.contains("A"));
        assert!(d_sep_graph.nodes["C"].edges.contains("B"));
    }

    #[test]
    fn double_edge_test() {
        let mut base_graph = Graph::new();
        base_graph.add_node("A".to_string());
        base_graph.add_node("B".to_string());
        base_graph.add_node("C".to_string());

        base_graph.add_edge(&"A".to_string(), &"B".to_string());
        base_graph.add_edge(&"C".to_string(), &"B".to_string());

        let mut d_sep_graph = Graph::new();
        d_sep_graph.add_node("B".to_string());
        d_sep_graph.add_node("C".to_string());

        add_parents(&mut d_sep_graph, &base_graph);
        moralize(&mut d_sep_graph);
        double_edge(&mut d_sep_graph);

        assert!(d_sep_graph.nodes["A"].edges.contains("B"));
        assert!(d_sep_graph.nodes["A"].edges.contains("C"));

        assert!(d_sep_graph.nodes["C"].edges.contains("A"));
        assert!(d_sep_graph.nodes["C"].edges.contains("B"));

        assert!(d_sep_graph.nodes["B"].edges.contains("A"));
        assert!(d_sep_graph.nodes["B"].edges.contains("C"));
    }

    #[test]
    fn test_not_d_separation() {
        let mut graph = Graph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());

        graph.add_edge(&"A".to_string(), &"B".to_string());
        graph.add_edge(&"C".to_string(), &"B".to_string());

        assert!(is_d_separated(
            &graph,
            &"A".to_string(),
            &"B".to_string(),
            &vec![]
        ));

        assert!(!is_d_separated(
            &graph,
            &"A".to_string(),
            &"C".to_string(),
            &vec![]
        ));

        assert!(is_d_separated(
            &graph,
            &"A".to_string(),
            &"C".to_string(),
            &vec!["B".to_string()]
        ));
    }

    #[test]
    fn test_d_separation() {
        let mut graph = Graph::new();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());
        graph.add_node("D".to_string());

        graph.add_edge(&"A".to_string(), &"B".to_string());
        graph.add_edge(&"C".to_string(), &"B".to_string());
        graph.add_edge(&"D".to_string(), &"C".to_string());

        assert!(is_d_separated(
            &graph,
            &"A".to_string(),
            &"C".to_string(),
            &vec!["B".to_string()]
        ));

        assert!(is_d_separated(
            &graph,
            &"A".to_string(),
            &"D".to_string(),
            &vec!["B".to_string()]
        ));
    }
}
