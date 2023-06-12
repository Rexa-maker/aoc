use lazy_static::lazy_static;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("{}", shortest_distance(input));
}

fn shortest_distance(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<source>\w+) to (?P<destination>\w+) = (?P<distance>\d+)").unwrap();
    }

    let mut shortest = usize::MAX;

    let mut hash: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph: UnGraph<String, usize> = UnGraph::new_undirected();

    for capture in RE.captures_iter(input) {
        let names = vec![
            capture.name("source").unwrap().as_str().to_string(),
            capture.name("destination").unwrap().as_str().to_string(),
        ];
        let distance = capture
            .name("distance")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let mut nodes = Vec::<NodeIndex>::new();
        for name in names {
            println!("DEBUGAX {name} {:?}", hash);
            if hash.contains_key(&name) {
                nodes.push(*hash.get(&name).unwrap());
            } else {
                let index = graph.add_node(name.clone());
                hash.insert(name, index);
                nodes.push(index);
            }
        }

        graph.add_edge(*nodes.first().unwrap(), *nodes.last().unwrap(), distance);
    }

    println!("{:?} {:?}", hash, graph);
    let nodes = hash.into_iter().collect::<Vec<(String, NodeIndex)>>();
    for i in 0..nodes.len() - 1 {}
    let distances = dijkstra(&graph, any_node, None, |edge| *edge.weight());

    for (_, distance) in distances {
        println!("DEBUGAX {distance}");
        if distance > 0 && distance < shortest {
            shortest = distance;
        }
    }

    shortest
}

#[test]
fn examples() {
    let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    assert_eq!(shortest_distance(input), 605);
}
