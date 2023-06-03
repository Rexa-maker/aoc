fn main() {
    let input = include_str!("input");
    let data = input_to_data(input);
    println!("{} {}", max_calories(&data), 0);
}

fn input_to_data(input: &str) -> Vec<Vec<u32>> {
    let mut data = vec![];
    let mut elf = vec![];
    for line in input.lines() {
        if line.is_empty() {
            data.push(elf.clone());
            elf = vec![];
        } else {
            elf.push(line.parse().unwrap())
        }

    }
    data.clone()
}

fn max_calories(data: &Vec<Vec<u32>>) -> u32 {
    let mut max_calories = 0;

    for elf in data {
        let calories = elf.iter().fold(0, |tally, x| tally + x);
        if calories > max_calories {
            max_calories = calories;
        }
    }

    max_calories
}

#[test]
fn examples() {
    assert_eq!(max_calories(&input_to_data("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
")), 24000);
}
