fn main() {
    let input = include_str!("input");
    let data = input_to_data(input);
    println!("{} {}", mult_of_sum_to_2020(&data), 0);
}

fn input_to_data(input: &str) -> Vec<u32> {
    input.lines().map(|val| val.parse().unwrap()).collect()
}

fn mult_of_sum_to_2020(data: &Vec<u32>) -> u32 {
    for (i, left) in data.iter().enumerate() {
        for right in &data[i+1..] {
            if left + right == 2020 {
                return left * right;
            }
        }
    }
    0
}

#[test]
fn examples() {
    assert_eq!(mult_of_sum_to_2020(&input_to_data("1721
979
366
299
675
1456")), 514579);
}
