use std::fs;
use std::env;
use anyhow::{anyhow, Result};

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
    fn process_edge(&mut self, start: NodeId, end: NodeId) {
        // Point from lesser -> greater
        if start > end {
            let idx = self.adjs[start]
                .iter()
                .position(|&f| f == end)
                .unwrap();

            self.adjs[start].remove(idx);
        } else {
            let idx = self.adjs[end]
                .iter()
                .position(|&f| f == start)
                .unwrap();

            self.adjs[end].remove(idx);
        }
    }
    pub fn to_dag(&mut self) {
        for (i, node) in self.adjs.clone().iter().enumerate(){
            for connection in node {
                if *connection < i {
                    continue //we have already done 0->1 so 1->0 will not be there
                }
                self.process_edge(i, *connection);
            }
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[Usage] ./demo_graph_solver[.exe] {{FILE_PATH}}");
        return Err(anyhow!("invalid use"));
    }
    let mut graph = Graph::from_file(args[1].as_str());
    graph.to_dag();
    println!("{:#?}", graph.adjs);

    return Ok(());
}
