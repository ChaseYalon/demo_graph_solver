use std::collections::HashMap;
use std::fs;
use std::env;
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node {
    id: usize,
    color: Option<usize>,
}
impl Node {
    pub fn new(id: usize) -> Node {
        return Node { id, color: None };
    }
}
#[derive(Debug)]
struct UnDirectedGraph {
    nodes: Vec<Node>,
    connections: HashMap<Node, Vec<Node>>,
}

impl UnDirectedGraph {
    pub fn from_file(path: &str) -> UnDirectedGraph {
        let text = fs::read_to_string(path).unwrap();
        let pairs: Vec<Vec<usize>> = text
            .split("\n")
            .map(|r| {
                r.trim()
                    .split("-")
                    .map(|r| r.trim().parse().unwrap())
                    .collect()
            })
            .collect();
        let mut graph = UnDirectedGraph {
            nodes: vec![],
            connections: HashMap::new(),
        };
        for pair in pairs {
            let mut n1 = Node::new(pair[0]);
            let mut n2 = Node::new(pair[1]);
            if !graph.nodes.iter().any(|n| n.id == pair[0]) {
                graph.nodes.push(n1.clone());
            } else {
                n1 = graph.nodes.iter().find(|n| n.id == pair[0]).unwrap().clone();
            }
            if !graph.nodes.iter().any(|n| n.id == pair[1]) {
                graph.nodes.push(n2.clone());
            } else {
                n2 = graph.nodes.iter().find(|n| n.id == pair[1]).unwrap().clone();
            }
            let n1_conns = graph.connections.entry(n1.clone()).or_insert(vec![]);
            n1_conns.push(n2.clone());
            let n2_conns = graph.connections.entry(n2).or_insert(vec![]);
            n2_conns.push(n1);
        }

        return graph;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", UnDirectedGraph::from_file(args[1].as_str()));
}
