fn main() {
    let input = include_str!("input");
    let data = input_to_data(input);
    println!("{} {}", how_many_increases(&data), 0);
}

fn input_to_data(input: &str) -> Vec<u32> {
    input.lines().map(|val| val.parse().unwrap()).collect()
}

fn how_many_increases(data: &Vec<u32>) -> u32 {
    let mut increases = 0;
    let mut last_value: u32 = 0;

    for depth in data {
        if last_value != 0 && *depth > last_value {
            increases += 1;
        }
        last_value = *depth;
    }

    increases
}

#[test]
fn examples() {
    assert_eq!(how_many_increases(&input_to_data("199
200
208
210
200
207
240
269
260
263")), 7);
}
