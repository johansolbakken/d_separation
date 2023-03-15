use d_separation::{is_d_separated, Graph};

fn main() {
    let mut graph = Graph::new();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.add_edge(&"A".to_string(), &"B".to_string());
    graph.add_edge(&"C".to_string(), &"B".to_string());

    assert!(!is_d_separated(
        &graph,
        &"A".to_string(),
        &"C".to_string(),
        &vec![]
    ));
}
