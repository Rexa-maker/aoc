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
    let mut skip_next_backslash = false;

    let mut total = 0;

    for (i, c) in chars.char_indices() {
        if i == 0 || i == chars_len - 1 {
            // " -> nothing = 1 difference
            total += 1;
        } else if c == '\\' {
            if skip_next_backslash {
                skip_next_backslash = false;
                continue;
            }

            let next_char = chars.chars().nth(i + 1).unwrap();
            if next_char == 'x' {
                total += 3; // \xXX -> A = 3 difference
            } else {
                total += 1; // \X -> X = 1 difference
                if next_char == '\\' {
                    // \\ -> \ = 1 difference but we must ignore the second \
                    skip_next_backslash = true;
                }
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
    assert_eq!(line_difference_code_ram("\"\\\\\""), 3);
}
