use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");
    println!("{}", shortest_distance(input));
}

fn tsp(graph: &HashMap<(&str, &str), usize>, start: &str) -> usize {
    let mut stack: Vec<(&str, Vec<&str>, usize)> = Vec::new();
    let mut shortest = std::usize::MAX;

    stack.push((start, vec![start], 0));

    while let Some((current, visited, distance)) = stack.pop() {
        if visited.len() == graph.len() {
            shortest = shortest.min(distance);
        } else {
            for ((a, b), path_len) in graph
                .iter()
                .filter(|((a, b), _)| *a == current || *b == current)
            {
                let next = if *a == current { *b } else { *a };
                if visited.contains(&next) {
                    continue;
                }

                let mut new_visited = visited.clone();
                new_visited.push(next);
                stack.push((next, new_visited, distance + path_len));
            }
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
        let trip = tsp(&graph, &city);
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
