use lazy_static::lazy_static;
use permutohedron;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("{} {}", shortest_distance(input), longest_distance(input));
}

enum WantedSolution {
    Longest,
    Shortest,
}

fn tsp(graph: &HashMap<(&str, &str), usize>, wanted_solution: WantedSolution) -> usize {
    let mut cities: Vec<&str> = graph
        .keys()
        .map(|(a, b)| vec![*a, *b])
        .flatten()
        .collect::<Vec<&str>>();
    cities.sort();
    cities.dedup();

    let mut best_distance = match wanted_solution {
        WantedSolution::Longest => 0,
        WantedSolution::Shortest => usize::max_value(),
    };

    'try_permutation: for permutation in permutohedron::Heap::new(&mut cities) {
        let mut distance = 0;
        let mut src = permutation[0];

        for dst in permutation.iter().skip(1) {
            let key1 = &(src, *dst);
            let key2 = &(*dst, src);
            if graph.contains_key(key1) {
                distance += graph.get(key1).unwrap();
            } else if graph.contains_key(key2) {
                distance += graph.get(key2).unwrap();
            } else {
                // This permutation is invalid, try the next one
                continue 'try_permutation;
            }
            src = *dst;
        }

        best_distance = match wanted_solution {
            WantedSolution::Longest => usize::max(best_distance, distance),
            WantedSolution::Shortest => usize::min(best_distance, distance),
        };
    }

    best_distance
}

fn parse_input(input: &str) -> HashMap<(&str, &str), usize> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<source>\w+) to (?P<destination>\w+) = (?P<distance>\d+)").unwrap();
    }

    let mut graph: HashMap<(&str, &str), usize> = HashMap::new();

    for capture in RE.captures_iter(input) {
        let src = capture.name("source").unwrap().as_str();
        let dst = capture.name("destination").unwrap().as_str();
        let distance = capture
            .name("distance")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        graph.insert((&src, &dst), distance);
    }

    graph
}

fn shortest_distance(input: &str) -> usize {
    let graph = parse_input(input);

    tsp(&graph, WantedSolution::Shortest)
}

fn longest_distance(input: &str) -> usize {
    let graph = parse_input(input);

    tsp(&graph, WantedSolution::Longest)
}

#[test]
fn examples() {
    let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    assert_eq!(shortest_distance(input), 605);
    assert_eq!(longest_distance(input), 982);
}
