fn main() {
    let input = include_str!("input");
    println!("{}", how_many_combinations(input, 150));
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

struct Combinations {
    containers: Vec<u32>,
    used_containers_mask: u32,
}

impl Combinations {
    fn new(containers: Vec<u32>) -> Self {
        Self {
            containers,
            used_containers_mask: 0,
        }
    }

    fn can_fit_total(&self, total: u32) -> bool {
        let mut sum = 0;
        for i in 0..self.containers.len() {
            if self.used_containers_mask & (1 << i) != 0 {
                sum += self.containers[i];
            }
        }
        sum == total
    }
}

fn how_many_combinations(input: &str, total: u32) -> usize {
    let containers = parse_input(input);
    assert!(containers.len() <= 32);

    let mut combinations = Combinations::new(containers);
    let mut count = 0;

    while combinations.used_containers_mask < (1 << combinations.containers.len()) {
        if combinations.can_fit_total(total) {
            count += 1;
        }
        combinations.used_containers_mask += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_many_combinations() {
        let input = "20
15
10
5
5";
        assert_eq!(how_many_combinations(input, 25), 4);
    }
}
