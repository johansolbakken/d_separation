# D-separation

Project where I implemented an algorithm that checks if a graph is d-separated in Rust.

```rust
use d_separation::{is_d_separated, Graph};

fn main() {
    let a = "A".to_string();
    let b = "B".to_string();
    let c = "C".to_string();

    let mut graph = Graph::new();
    graph.add_node(&a);
    graph.add_node(&b);
    graph.add_node(&c);

    graph.add_edge(&a, &b);
    graph.add_edge(&c, &b);

    assert!(is_d_separated(&graph, &a, &c, &vec![c.clone()]));
}

```
