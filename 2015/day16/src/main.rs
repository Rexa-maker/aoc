use std::collections::HashMap;

struct Sue {
    name: u16,
    coumpounds: HashMap<&'static str, u8>,
}
static COUMPOUNDS: [&str; 10] = [
    "children",
    "cats",
    "samoyeds",
    "pomeranians",
    "akitas",
    "vizslas",
    "goldfish",
    "trees",
    "cars",
    "perfumes",
];

impl Sue {
    // Input line example: "Sue 1: goldfish: 6, trees: 9, akitas: 0"
    fn parse(line: &str) -> Sue {
        // Returns the name of the coumpound within the static array
        fn coumpound_valid(name: &str) -> Option<&'static str> {
            for coumpound in COUMPOUNDS.iter() {
                if name == *coumpound {
                    return Some(*coumpound);
                }
            }
            None
        }

        let (name, coumpounds_str) = line.split_once(": ").unwrap();
        let sue_name = name[4..].parse().unwrap();

        let mut coumpounds = HashMap::new();

        for compound in coumpounds_str.split(", ") {
            let mut parts = compound.split(": ");
            let cmpnd_name = coumpound_valid(parts.next().unwrap()).unwrap();
            let value = parts.next().unwrap().parse().unwrap();

            coumpounds.insert(cmpnd_name, value);
        }

        Sue {
            name: sue_name,
            coumpounds,
        }
    }
}

// Original Sue the message leads us to find
fn message() -> Sue {
    Sue::parse(
        "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, \
               vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1",
    )
}

fn which_sue(input: &str) -> u16 {
    let message = message();
    let mut sues = Vec::new();

    for line in input.lines() {
        sues.push(Sue::parse(line));
    }

    for sue in sues {
        let mut valid = true;
        for (coumpound, value) in message.coumpounds.iter() {
            if let Some(sue_value) = sue.coumpounds.get(coumpound) {
                if *value != *sue_value {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            return sue.name;
        }
    }

    0
}

fn which_real_sue(input: &str) -> u16 {
    let message = message();
    let mut sues = Vec::new();

    for line in input.lines() {
        sues.push(Sue::parse(line));
    }

    for sue in sues {
        let mut valid = true;
        for (&coumpound, value) in message.coumpounds.iter() {
            if let Some(sue_value) = sue.coumpounds.get(coumpound) {
                match coumpound {
                    "cats" | "trees" => {
                        if *value >= *sue_value {
                            valid = false;
                            break;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if *value <= *sue_value {
                            valid = false;
                            break;
                        }
                    }
                    _ => {
                        if *value != *sue_value {
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }

        if valid {
            return sue.name;
        }
    }

    0
}

fn main() {
    let input = include_str!("input");
    println!("{} {}", which_sue(input), which_real_sue(input));
}
