use iter_tools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input");
    let mut lights1 = LightsMatrix1::new();
    let mut lights2 = LightsMatrix2::new();
    for line in input.lines() {
        lights1.instruction(line);
        lights2.instruction(line);
    }
    println!("{} {}", lights1.how_many_lit(), lights2.brightness());
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    action: Action,
    iterator: Vec<(usize, usize)>,
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

        let mut v = Vec::new();
        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                v.push((x, y));
            }
        }

        Instruction {
            action: action,
            iterator: v,
        }
    }
}

struct LightsMatrix1 {
    matrix: Vec<Vec<bool>>,
}

impl LightsMatrix1 {
    fn new() -> Self {
        Self {
            matrix: vec![vec![false; 1000]; 1000],
        }
    }

    fn instruction(self: &mut Self, line: &str) -> &Self {
        let instruction = Instruction::parse(line);
        for (x, y) in instruction.iterator.iter() {
            match instruction.action {
                Action::Toggle => {
                    self.matrix[*x][*y] = !self.matrix[*x][*y];
                }
                Action::TurnOff => {
                    self.matrix[*x][*y] = false;
                }
                Action::TurnOn => {
                    self.matrix[*x][*y] = true;
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

struct LightsMatrix2 {
    matrix: Vec<Vec<u32>>,
}

impl LightsMatrix2 {
    fn new() -> Self {
        Self {
            matrix: vec![vec![0; 1000]; 1000],
        }
    }

    fn instruction(self: &mut Self, line: &str) -> &Self {
        let instruction = Instruction::parse(line);
        for (x, y) in instruction.iterator.iter() {
            match instruction.action {
                Action::Toggle => {
                    self.matrix[*x][*y] += 2;
                }
                Action::TurnOff => {
                    if self.matrix[*x][*y] > 0 {
                        self.matrix[*x][*y] -= 1;
                    }
                }
                Action::TurnOn => {
                    self.matrix[*x][*y] += 1;
                }
            }
        }
        self
    }

    fn brightness(self: &Self) -> u32 {
        self.matrix.iter().fold(0, |tally_lines, line| {
            tally_lines + line.iter().sum::<u32>()
        })
    }
}

#[test]
fn examples() {
    let mut lights1 = LightsMatrix1::new();
    assert_eq!(
        lights1
            .instruction("turn on 0,0 through 999,999")
            .how_many_lit(),
        1000000
    );
    assert_eq!(
        lights1
            .instruction("toggle 0,0 through 999,0")
            .how_many_lit(),
        999000
    );
    assert_eq!(
        lights1
            .instruction("turn off 499,499 through 500,500")
            .how_many_lit(),
        998996
    );

    let mut lights2 = LightsMatrix2::new();
    assert_eq!(
        lights2.instruction("turn on 0,0 through 0,0").brightness(),
        1
    );
    assert_eq!(
        lights2
            .instruction("toggle 0,0 through 999,999")
            .brightness(),
        2000001
    );
}
