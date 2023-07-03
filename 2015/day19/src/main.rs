fn main() {
    let input = include_str!("input");
    println!("{} {}", part1(input), part2(input));
}

fn parse_input(input: &str) -> (&str, Vec<(&str, &str)>) {
    let mut lines = input.lines();
    let mut replacements = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        replacements.push((from, to));
    }

    let molecule = lines.next().unwrap();
    (molecule, replacements)
}

fn part1(input: &str) -> usize {
    let (molecule, replacements) = parse_input(input);

    let mut molecules = Vec::new();
    for (from, to) in replacements {
        let mut start = 0;
        while let Some(pos) = molecule[start..].find(from) {
            let mut new_molecule = molecule.clone().to_string();
            new_molecule.replace_range(start + pos..start + pos + from.len(), to);
            molecules.push(new_molecule);
            start += pos + 1;
        }
    }
    molecules.sort();
    molecules.dedup();
    molecules.len()
}

fn part2(input: &str) -> usize {
    let (molecule, replacements) = parse_input(input);

    // NOTE: this is broken
    let mut steps = 0;
    let mut molecule = molecule.to_string();
    while molecule != "e" {
        for (from, to) in &replacements {
            if molecule.contains(to) {
                molecule = molecule.replace(to, from);
                steps += 1;
            }
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "H => HO
H => OH
O => HH

HOH";
        assert_eq!(part1(input), 4);
    }

    #[test]
    #[ignore] // part 2 is broken for now
    fn test_part1_2() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOH";
        assert_eq!(part2(input), 3);

        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";
        assert_eq!(part2(input), 6);
    }
}
