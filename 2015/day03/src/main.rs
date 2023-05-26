use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    println!("{} {}", how_many_presents(input), how_many_presents2(input));
}

fn char2direction(direction: char, position: &mut (i32, i32)) {
    match direction {
        '<' => position.0 -= 1,
        '>' => position.0 += 1,
        '^' => position.1 -= 1,
        'v' => position.1 += 1,
        _ => (),
    }
}

fn how_many_presents(input: &str) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut position = (0, 0);
    visited.insert(position);

    for direction in input.chars() {
        char2direction(direction, &mut position);
        visited.insert(position);
    }

    visited.len() as u32
}

fn how_many_presents2(input: &str) -> u32 {
    let mut santa_moving = true;
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut positions = ((0, 0), (0, 0));
    visited.insert(positions.0);

    for direction in input.chars() {
        let position = if santa_moving {
            &mut positions.0
        } else {
            &mut positions.1
        };
        char2direction(direction, position);
        visited.insert(*position);
        santa_moving = !santa_moving;
    }

    visited.len() as u32
}

#[test]
fn examples() {
    assert_eq!(how_many_presents(">"), 2);
    assert_eq!(how_many_presents("^>v<"), 4);
    assert_eq!(how_many_presents("^v^v^v^v^v"), 2);

    assert_eq!(how_many_presents2("^v"), 3);
    assert_eq!(how_many_presents2("^>v<"), 3);
    assert_eq!(how_many_presents2("^v^v^v^v^v"), 11);
}
