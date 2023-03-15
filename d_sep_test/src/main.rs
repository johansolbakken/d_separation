use d_separation::{add_parents, moralize, Graph};

fn main() {
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

    println!("{:#?}", d_sep_graph);
}
