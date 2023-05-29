fn main() {
    println!("{}", double_sum(include_str!("input")));
}

fn double_sum(input: &str) -> u32 {
    let mut fist_char: char = '\0';
    let mut prev_char: char = '\0';
    let mut tally = 0;

    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            fist_char = c;
        }

        if prev_char == c {
            tally += c.to_digit(10).unwrap();
        }

        prev_char = c;
    }

    if fist_char == prev_char {
        tally += fist_char.to_digit(10).unwrap();
    }

    tally
}

#[test]
fn example() {
    assert_eq!(double_sum("1122"), 3);
    assert_eq!(double_sum("1111"), 4);
    assert_eq!(double_sum("1234"), 0);
    assert_eq!(double_sum("91212129"), 9);
}
