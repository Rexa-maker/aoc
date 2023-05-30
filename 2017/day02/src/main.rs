fn main() {
    let input = include_str!("input");
    println!("{}", checksum(input));
}

fn checksum(input: &str) -> u32 {
    let mut tally = 0;
    for line in input.lines() {
        let mut values: Vec<u32> = line
            .split_whitespace()
            .map(|text| text.parse::<u32>().unwrap())
            .collect();
        values.sort();
        tally += values.last().unwrap() - values.first().unwrap();
    }
    tally
}

#[test]
fn day02_examples() {
    let example = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(checksum(example), 18);
}
