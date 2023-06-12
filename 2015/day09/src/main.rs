use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");
    println!("{}", shortest_distance(input));
}

// ChatGPT
fn tsp(graph: &HashMap<(&str, &str), usize>, current: &str, visited: &Vec<&str>) -> usize {
    // Base case: all locations have been visited
    if visited.len() == graph.len() {
        return 0;
    }

    // Recursive case: find the shortest path from the current location
    let mut shortest = std::usize::MAX;
    for ((a, b), distance) in graph
        .iter()
        .filter(|((a, b), _)| *a == current || *b == current)
    {
        let next = if *a == current { *b } else { *a };
        if visited.contains(&next) {
            continue;
        }

        let mut new_visited = visited.clone();
        new_visited.push(next);
        let path_len = distance + tsp(graph, next, &new_visited);
        if path_len < shortest {
            shortest = path_len;
        }
    }

    shortest
}

fn shortest_distance(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<source>\w+) to (?P<destination>\w+) = (?P<distance>\d+)").unwrap();
    }

    let mut graph: HashMap<(&str, &str), usize> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    for capture in RE.captures_iter(input) {
        let src = capture.name("source").unwrap().as_str();
        let dst = capture.name("destination").unwrap().as_str();
        let distance = capture
            .name("distance")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        cities.insert(&src);
        cities.insert(&dst);
        graph.insert((&src, &dst), distance);
    }

    let mut shortest = usize::MAX;

    for city in cities {
        let visited = vec![city];
        let trip = tsp(&graph, &city, &visited);
        if trip < shortest {
            shortest = trip;
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
