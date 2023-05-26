fn main() {
    let input = include_str!("input");
    println!("{} {}", tally_surfaces(input), tally_ribbon(input));
}

fn tally_surfaces(input: &str) -> u32 {
    let mut tally = 0;
    for line in input.lines() {
        tally += Box::new(line).total_needed();
    }
    tally
}

fn tally_ribbon(input: &str) -> u32 {
    let mut tally = 0;
    for line in input.lines() {
        tally += Box::new(line).ribbon_feet();
    }
    tally
}

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl Box {
    fn new(line: &str) -> Box {
        let dimensions: Vec<u32> = line.split("x").map(|x| x.parse().unwrap()).collect();
        Box {
            l: dimensions[0],
            w: dimensions[1],
            h: dimensions[2],
        }
    }

    fn surface(self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn total_needed(self) -> u32 {
        let mut sorted = vec![self.l, self.w, self.h];
        sorted.sort();
        self.surface() + sorted[0] * sorted[1]
    }

    fn ribbon_feet(self) -> u32 {
        let mut sorted = vec![self.l, self.w, self.h];
        sorted.sort();

        2 * (sorted[0] + sorted[1]) + self.l * self.w * self.h
    }
}

#[test]
fn examples() {
    assert_eq!(Box::new("2x3x4").surface(), 52);
    assert_eq!(Box::new("2x3x4").total_needed(), 58);
    assert_eq!(Box::new("1x1x10").surface(), 42);
    assert_eq!(Box::new("1x1x10").total_needed(), 43);

    assert_eq!(Box::new("2x3x4").ribbon_feet(), 34);
    assert_eq!(Box::new("1x1x10").ribbon_feet(), 14);
}
