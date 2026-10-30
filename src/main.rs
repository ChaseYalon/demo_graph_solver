use std::fs;
use std::env;
type NodeId = usize;

#[derive(Debug)]
struct Graph {
    adjs: Vec<Vec<NodeId>>, //adjs[u] = all of the nodes adjacent to u
    colors: Vec<()> //later colors[u] will be the color of node u
}
impl Graph {
    ///graphs are assumed to be dense and bi-directional when loaded
    pub fn from_file(path: &str) -> Graph {
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
        let max_id = pairs.iter().flatten().max().unwrap();
        let mut graph = Graph {
            adjs: vec![],
            colors: vec![]
        };
        graph.adjs.resize(*max_id + 1, vec![]);
        graph.colors.resize(*max_id + 1, ());
        for pair in pairs {
            graph.adjs[pair[0]].push(pair[1]);
            graph.adjs[pair[1]].push(pair[0]);
        }
        return graph;
    }
    pub fn to_dag(&mut self) {
        unimplemented!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut graph = Graph::from_file(args[1].as_str());
    println!("{:#?}", graph);
}
