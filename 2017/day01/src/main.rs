fn main() {
    let input = include_str!("input");
    println!("{} {}", double_sum(input), halfway_sum(input));
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

fn halfway_sum(input: &str) -> u32 {
    let half = input.len() / 2;
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut tally = 0;
    for i in 0..half {
        if digits[i] == digits[i + half] {
            tally += digits[i] * 2;
        }
    }
    tally
}

#[test]
fn example() {
    assert_eq!(double_sum("1122"), 3);
    assert_eq!(double_sum("1111"), 4);
    assert_eq!(double_sum("1234"), 0);
    assert_eq!(double_sum("91212129"), 9);

    assert_eq!(halfway_sum("1212"), 6);
    assert_eq!(halfway_sum("1221"), 0);
    assert_eq!(halfway_sum("123425"), 4);
    assert_eq!(halfway_sum("123123"), 12);
    assert_eq!(halfway_sum("12131415"), 4);
}
