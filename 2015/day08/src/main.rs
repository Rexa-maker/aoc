fn main() {
    let input = include_str!("input");
    println!("{} {}", difference_code_ram(input), 0);
}

fn difference_code_ram(input: &str) -> usize {
    input
        .lines()
        .fold(0, |tally, line| tally + line_difference_code_ram(line))
}

fn line_difference_code_ram(line: &str) -> usize {
    let chars = line.to_string();
    let chars_len = chars.len();

    let mut total = 0;

    for (i, c) in chars.char_indices() {
        if i == 0 || i == chars_len - 1 {
            assert_eq!(c, '\"');
            total += 1;
        } else if c == '\\' {
            if chars.chars().nth(i + 1).unwrap() == 'x' {
                total += 3;
            } else {
                total += 1;
            }
        }
    }

    total
}

#[test]
fn examples() {
    let input = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";
    assert_eq!(difference_code_ram(input), 12);
}
