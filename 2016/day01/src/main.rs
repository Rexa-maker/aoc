use std::collections::HashSet;

fn main() {
    let answer = how_many_blocks(include_str!("input"));
    println!("{} {}", answer.0, answer.1);
}

enum Cardinal {
    North,
    East,
    South,
    West,
}

fn turn(rotation: char, facing: &mut Cardinal) {
    match facing {
        Cardinal::North => match rotation {
            'R' => *facing = Cardinal::East,
            'L' => *facing = Cardinal::West,
            _ => (),
        },
        Cardinal::East => match rotation {
            'R' => *facing = Cardinal::South,
            'L' => *facing = Cardinal::North,
            _ => (),
        },
        Cardinal::South => match rotation {
            'R' => *facing = Cardinal::West,
            'L' => *facing = Cardinal::East,
            _ => (),
        },
        Cardinal::West => match rotation {
            'R' => *facing = Cardinal::North,
            'L' => *facing = Cardinal::South,
            _ => (),
        },
    }
}

fn advance(
    facing: &Cardinal,
    distance: i32,
    position: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> (u32, (i32, i32)) {
    let mut distance_visited_twice = 0;
    let mut new_position = position;

    for _ in 0..distance {
        match facing {
            Cardinal::North => new_position.0 += 1,
            Cardinal::East => new_position.1 += 1,
            Cardinal::South => new_position.0 -= 1,
            Cardinal::West => new_position.1 -= 1,
        };

        if !visited.insert(new_position) {
            let new_distance = manhattan_distance(&new_position);
            if new_distance < distance_visited_twice || distance_visited_twice == 0 {
                distance_visited_twice = new_distance;
            }
        }
    }
    (distance_visited_twice, new_position)
}

fn manhattan_distance(position: &(i32, i32)) -> u32 {
    (position.0.abs() + position.1.abs()) as u32
}

/// Returns (shortest distance to the first location visited twice, shortest distance after doing all the instructions)
fn how_many_blocks(input: &str) -> (u32, u32) {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut position: (i32, i32) = (0, 0);
    let mut facing = Cardinal::North;
    let mut result: (u32, u32) = (0, 0);

    visited.insert(position);

    for movement in input.split(", ") {
        let mut chars = movement.chars();

        let rotation = chars.next().unwrap();
        let distance: u32 = chars.collect::<String>().parse().unwrap();
        turn(rotation, &mut facing);
        let distance_visited_twice;
        (distance_visited_twice, position) =
            advance(&facing, distance as i32, position, &mut visited);
        if distance_visited_twice != 0 && (result.1 == 0 || result.1 > distance_visited_twice) {
            result.1 = distance_visited_twice;
        }
    }
    result.0 = manhattan_distance(&position);
    result
}

#[test]
fn day01_examples() {
    assert_eq!(how_many_blocks("R2, L3").0, 5);
    assert_eq!(how_many_blocks("R2, R2, R2").0, 2);
    assert_eq!(how_many_blocks("R5, L5, R5, R3").0, 12);

    assert_eq!(how_many_blocks("R8, R4, R4, R8").1, 4);
}
