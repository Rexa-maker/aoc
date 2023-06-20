fn main() {
    static INPUT: &str = include_str!("input");
    println!(
        "{} {}",
        max_happiness(INPUT, false),
        max_happiness(INPUT, true)
    );
}

fn max_happiness(input: &str, add_myself: bool) -> i32 {
    let mut people = std::collections::HashSet::new();
    let mut happiness_map = std::collections::HashMap::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let person = words.next().unwrap();
        // Skip "would" then get "gain" or "lose
        let sign = words.nth(1).unwrap();
        let amount = words.next().unwrap().parse::<i32>().unwrap();
        // Skip "happiness units by sitting next to" then get the number
        let next_to = words.nth(6).unwrap().trim_end_matches('.');

        let amount = match sign {
            "gain" => amount,
            "lose" => -amount,
            _ => unreachable!(),
        };

        people.insert(person);
        happiness_map.insert((person, next_to), amount);
    }

    let mut people: Vec<_> = people.into_iter().collect();

    if add_myself {
        for person in people.iter() {
            happiness_map.insert((person, "MOI"), 0);
            happiness_map.insert(("MOI", person), 0);
        }
        people.insert("MOI");
    }

    let mut max_happiness = 0;

    for permutation in permutohedron::Heap::new(&mut people) {
        let mut happiness: i32 = 0;

        for (i, person) in permutation.iter().enumerate() {
            let next_to = if i == 0 {
                permutation[permutation.len() - 1]
            } else {
                permutation[i - 1]
            };

            happiness += happiness_map.get(&(person, next_to)).unwrap();
            happiness += happiness_map.get(&(next_to, person)).unwrap();
        }

        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }

    max_happiness
}

#[test]
fn examples() {
    let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
    assert_eq!(max_happiness(input, false), 330);
}
