fn main() {
    let input = include_str!("input");
    let checksums = checksums(input);
    println!("{} {}", checksums.0, checksums.1);
}

fn checksums(input: &str) -> (u32, u32) {
    let mut tally = (0, 0);
    for line in input.lines() {
        let mut values: Vec<u32> = line
            .split_whitespace()
            .map(|text| text.parse::<u32>().unwrap())
            .collect();
        values.sort();
        tally.0 += values.last().unwrap() - values.first().unwrap();

        'outer: for i in 0..values.len() / 2 + 1 {
            for j in i + 1..values.len() {
                if values[j] % values[i] == 0 {
                    tally.1 += (values[j] / values[i]) as u32;
                    break 'outer;
                }
            }
        }
    }
    tally
}

#[test]
fn day02_examples() {
    let mut example = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(checksums(example).0, 18);

    example = "5 9 2 8
9 4 7 3
3 8 6 5";
    assert_eq!(checksums(example).1, 9);
}
