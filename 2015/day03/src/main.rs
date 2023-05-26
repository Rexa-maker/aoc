use std::collections::HashSet;

fn main() {
    println!("{}", how_many_presents(include_str!("input")));
}

fn how_many_presents(input: &str) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut position = (0, 0);
    visited.insert(position);

    for direction in input.chars() {
        match direction {
            '<' => {
                position.0 -= 1;
            }
            '>' => {
                position.0 += 1;
            }
            '^' => {
                position.1 -= 1;
            }
            'v' => {
                position.1 += 1;
            }
            _ => {}
        }
        visited.insert(position);
    }

    visited.len() as u32
}

#[test]
fn examples() {
    assert_eq!(how_many_presents(">"), 2);
    assert_eq!(how_many_presents("^>v<"), 4);
    assert_eq!(how_many_presents("^v^v^v^v^v"), 2);
}
