use std::{
    collections::HashSet,
    env, fs,
    process::{self},
};

use d_separation::{is_d_separated, Graph};
use fancy_regex::Regex;

#[derive(Debug)]
struct Intermediate {
    nodes: HashSet<String>,
    edges: HashSet<(String, String)>,
    calculations: Vec<String>,
}

fn compile(source: String) -> Intermediate {
    let mut intermediate = Intermediate {
        nodes: HashSet::new(),
        edges: HashSet::new(),
        calculations: Vec::new(),
    };

    let identifier = r"[a-zA-Z]([a-zA-Z0-9]|\_)*";

    let node_regex =
        Regex::new(format!(r"node {}( {})*", identifier, identifier).as_str()).unwrap();
    let edge_regex = Regex::new(format!(r"{} -> {}", identifier, identifier).as_str()).unwrap();
    let calc_regex = Regex::new(
        format!(
            r"calc {} {}((/( {})+)|)",
            identifier, identifier, identifier
        )
        .as_str(),
    )
    .unwrap();

    for line in source.lines() {
        if node_regex.is_match(line).unwrap() {
            let list = line.split(" ").collect::<Vec<&str>>();
            for i in 1..list.len() {
                intermediate.nodes.insert(list[i].to_string());
            }
        }

        if edge_regex.is_match(line).unwrap() {
            let list = line.split(" ").collect::<Vec<&str>>();
            let from = list[0].to_string();
            let to = list[2].to_string();
            intermediate.edges.insert((from, to));
        }

        if calc_regex.is_match(line).unwrap() {
            let list = line.split(" ").collect::<Vec<&str>>();
            let list = list[1..].to_vec();
            intermediate.calculations.push(list.join(" "));
        }
    }

    intermediate
}

fn build_graph(intermediate: &Intermediate) -> Graph {
    let mut graph = Graph::new();

    for node in intermediate.nodes.iter() {
        graph.add_node(&node);
    }

    for edge in intermediate.edges.iter() {
        graph.add_edge(&edge.0, &edge.1);
    }

    graph
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: dsepc <file>");
        process::exit(1);
    }

    let filepath = &args[1];
    let source = fs::read_to_string(filepath).expect("Error reading file");

    let intermediate = compile(source);

    let graph = build_graph(&intermediate);

    for calculation in intermediate.calculations.iter() {
        // split calculation into list of String
        let list = calculation
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let d_separated = if list.len() == 2 {
            is_d_separated(&graph, &list[0].to_string(), &list[1].to_string(), &vec![])
        } else if list.len() > 3 {
            is_d_separated(
                &graph,
                &list[0].to_string(),
                &list[1].to_string(),
                &list[3..].to_vec(),
            )
        } else {
            continue;
        };

        println!("{} -> {}", calculation, d_separated);
    }
}
