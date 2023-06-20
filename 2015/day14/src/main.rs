fn main() {
    static INPUT: &str = include_str!("input");
    println!("{}", longest_distance(INPUT, 2503));
}

struct Reindeer {
    speed: usize,
    duration: usize,
    rest: usize,
}

impl Reindeer {
    fn distance(&self, duration: usize) -> usize {
        let cycle = self.duration + self.rest;
        let cycles = duration / cycle;
        let remainder = duration % cycle;
        let mut distance = cycles * self.speed * self.duration;
        if remainder > self.duration {
            distance += self.speed * self.duration;
        } else {
            distance += self.speed * remainder;
        }
        distance
    }
}

fn longest_distance(input: &str, duration: usize) -> usize {
    let mut reindeers = Vec::new();
    for line in input.lines() {
        // "NAME can fly SPEED km/s for DURATION seconds, but then must rest for REST seconds."
        let mut words = line.split_whitespace();
        let speed = words.nth(3).unwrap().parse::<usize>().unwrap();
        let duration = words.nth(2).unwrap().parse::<usize>().unwrap();
        let rest = words.nth(6).unwrap().parse::<usize>().unwrap();
        reindeers.push(Reindeer {
            speed,
            duration,
            rest,
        });
    }

    let mut distances = reindeers
        .iter()
        .map(|r| r.distance(duration))
        .collect::<Vec<_>>();
    distances.sort();

    *distances.last().unwrap()
}

#[test]
fn examples() {
    let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
    assert_eq!(longest_distance(input, 1000), 1120);
}
