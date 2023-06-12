fn main() {
    let input = include_str!("input");
    println!(
        "{} {}",
        difference_code_ram(input),
        difference_new_code(input)
    );
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

fn difference_new_code(input: &str) -> usize {
    input
        .lines()
        .fold(0, |tally, line| tally + line_difference_new_code(line))
}

fn line_difference_new_code(line: &str) -> usize {
    let mut difference = 2; // 2 "

    for c in line.chars() {
        difference += match c {
            '\\' | '"' => 1,
            _ => 0,
        }
    }

    difference
}

#[test]
fn examples() {
    let input = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";
    assert_eq!(difference_code_ram(input), 12);
    assert_eq!(line_difference_code_ram("\"\\\\\""), 3);

    assert_eq!(difference_new_code(input), 19);
    assert_eq!(line_difference_new_code("\"\\\\\""), 6);
}
