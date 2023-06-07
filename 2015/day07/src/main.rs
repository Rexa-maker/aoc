use std::collections::HashMap;

fn main() {
    let mut wires: HashMap<String, Operation> = HashMap::new();
    let input = include_str!("input");
    input
        .lines()
        .map(|line| Operation::parse(line, &mut wires))
        .for_each(drop);
    println!("{} {}", resolve("a".to_string(), &mut wires), 0);
}

#[derive(Clone)]
enum Operation {
    INT(u16),
    AND(String, String),
    AND1(String),
    OR(String, String),
    RSHIFT(String, u8),
    LSHIFT(String, u8),
    NOT(String),
    IS(String),
}

impl Operation {
    /// Add an opearion to the list of wires
    fn parse(line: &str, wires: &mut HashMap<String, Operation>) {
        let words: Vec<&str> = line.split_whitespace().collect();
        let key = words.last().unwrap().to_string();
        let operation;

        assert_eq!(words[words.len() - 2], "->");

        if words.len() == 3 {
            if let Ok(a) = words[0].parse::<u16>() {
                operation = Operation::INT(a);
            } else {
                operation = Operation::IS(words[0].to_string());
            }
        } else if words[0] == "NOT" {
            assert_eq!(words.len(), 4);

            operation = Operation::NOT(words[1].to_string());
        } else {
            assert_eq!(words.len(), 5);

            if words[0] == "1" {
                assert_eq!(words[1], "AND");

                let right = words[2].to_string();
                operation = Operation::AND1(right);
            } else {
                let left = words[0].to_string();
                let right = words[2].to_string();

                operation = match words[1] {
                    "AND" => Operation::AND(left, right),
                    "OR" => Operation::OR(left, right),
                    "RSHIFT" => Operation::RSHIFT(left, right.parse::<u8>().unwrap()),
                    "LSHIFT" => Operation::LSHIFT(left, right.parse::<u8>().unwrap()),
                    _ => panic!("Cannot parse {line}"),
                }
            }
        }

        wires.insert(key, operation);
    }
}

fn resolve(wire: String, wires: &mut HashMap<String, Operation>) -> u16 {
    let operation = wires.get(&wire).unwrap().clone();

    let value = match operation {
        Operation::INT(a) => a,
        Operation::IS(wire) => resolve(wire, wires),
        Operation::NOT(wire) => !resolve(wire, wires),
        Operation::AND(left, right) => resolve(left, wires) & resolve(right, wires),
        Operation::AND1(right) => 1 & resolve(right, wires),
        Operation::OR(left, right) => resolve(left, wires) | resolve(right, wires),
        Operation::LSHIFT(wire, shift) => resolve(wire, wires) << shift,
        Operation::RSHIFT(wire, shift) => resolve(wire, wires) >> shift,
    };

    wires.insert(wire, Operation::INT(value));

    value
}

#[test]
fn examples() {
    let mut wires: HashMap<String, Operation> = HashMap::new();
    let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
    input
        .lines()
        .map(|line| Operation::parse(line, &mut wires))
        .for_each(drop);

    assert_eq!(resolve("d".to_string(), &mut wires), 72);
    assert_eq!(resolve("e".to_string(), &mut wires), 507);
    assert_eq!(resolve("f".to_string(), &mut wires), 492);
    assert_eq!(resolve("g".to_string(), &mut wires), 114);
    assert_eq!(resolve("h".to_string(), &mut wires), 65412);
    assert_eq!(resolve("i".to_string(), &mut wires), 65079);
    assert_eq!(resolve("x".to_string(), &mut wires), 123);
    assert_eq!(resolve("y".to_string(), &mut wires), 456);
}
