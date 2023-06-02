use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    let data = input_to_data(input);
    println!("{} {}", frequency(&data), loop_frequency_till_twice(&data));
}

fn input_to_data(input: &str) -> Vec<i32> {
    input.lines().map(|val| val.parse().unwrap()).collect()
}

fn frequency(data: &Vec<i32>) -> i32 {
    data.iter().fold(0, |tally: i32, x: &i32| tally + x)
}

fn loop_frequency_till_twice(data: &Vec<i32>) -> i32 {
    let mut seen = HashSet::<i32>::new();
    let mut tally = 0;
    seen.insert(tally);

    loop {
        for i in data {
            tally += i;
            if seen.insert(tally) != true {
                return tally;
            }
        }
    }
}

#[test]
fn examples() {
    assert_eq!(frequency(&input_to_data("+1\n-2\n+3\n+1")), 3);
    assert_eq!(frequency(&input_to_data("+1\n+1\n+1")), 3);
    assert_eq!(frequency(&input_to_data("+1\n+1\n-2")), 0);
    assert_eq!(frequency(&input_to_data("-1\n-2\n-3")), -6);

    assert_eq!(
        loop_frequency_till_twice(&input_to_data("+1\n-2\n+3\n+1")),
        2
    );
    assert_eq!(loop_frequency_till_twice(&input_to_data("+1\n-1")), 0);
    assert_eq!(
        loop_frequency_till_twice(&input_to_data("+3\n+3\n+4\n-2\n-4")),
        10
    );
    assert_eq!(
        loop_frequency_till_twice(&input_to_data("-6\n+3\n+8\n+5\n-6")),
        5
    );
    assert_eq!(
        loop_frequency_till_twice(&input_to_data("+7\n+7\n-2\n-7\n-4")),
        14
    );
}
