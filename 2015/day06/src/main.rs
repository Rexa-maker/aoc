use iter_tools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input");
    let mut lights = LightsMatrix::new();
    for line in input.lines() {
        lights.instruction(line);
    }
    println!("{} {}", lights.how_many_lit(), 0);
}

struct LightsMatrix {
    matrix: [[bool; 1000]; 1000],
}

impl LightsMatrix {
    fn new() -> LightsMatrix {
        LightsMatrix {
            matrix: [[false; 1000]; 1000],
        }
    }

    fn instruction(self: &mut Self, input: &str) -> &Self {
        enum Action {
            TurnOn,
            TurnOff,
            Toggle,
        }
        struct Instruction {
            action: Action,
            bounds: ((usize, usize), (usize, usize)),
        }

        impl Instruction {
            fn parse(line: &str) -> Instruction {
                lazy_static! {
                    static ref RE: Regex = Regex::new(
                        r"^(toggle|turn off|turn on) (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})"
                    )
                    .unwrap();
                }
                let binding = RE.captures(line).unwrap();
                let mut mat = binding.iter();

                mat.next(); // First match is the whole line

                let instruction_str = mat.next().unwrap().unwrap().as_str();
                let (x1, y1, x2, y2) = mat
                    .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let action = match instruction_str {
                    "turn on" => Action::TurnOn,
                    "turn off" => Action::TurnOff,
                    "toggle" => Action::Toggle,
                    _ => panic!("Failed to parse instruction {line}"),
                };
                Instruction {
                    action: action,
                    bounds: ((x1, y1), (x2, y2)),
                }
            }
        }

        for line in input.lines() {
            let instruction = Instruction::parse(line);
            for x in instruction.bounds.0 .0..instruction.bounds.1 .0 + 1 {
                for y in instruction.bounds.0 .1..instruction.bounds.1 .1 + 1 {
                    match instruction.action {
                        Action::Toggle => {
                            self.matrix[x][y] = !self.matrix[x][y];
                        }
                        Action::TurnOff => {
                            self.matrix[x][y] = false;
                        }
                        Action::TurnOn => {
                            self.matrix[x][y] = true;
                        }
                    }
                }
            }
        }
        self
    }

    fn how_many_lit(self: &Self) -> u32 {
        self.matrix.iter().fold(0, |tally_lines: u32, line| {
            tally_lines
                + line
                    .iter()
                    .fold(0, |tally, x| tally + (if *x { 1 } else { 0 }))
        })
    }
}

#[test]
fn examples() {
    let mut lights = LightsMatrix::new();
    assert_eq!(
        lights
            .instruction("turn on 0,0 through 999,999")
            .how_many_lit(),
        1000000
    );
    assert_eq!(
        lights
            .instruction("toggle 0,0 through 999,0")
            .how_many_lit(),
        999000
    );
    assert_eq!(
        lights
            .instruction("turn off 499,499 through 500,500")
            .how_many_lit(),
        998996
    );
}
