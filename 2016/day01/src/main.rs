use std::collections::HashSet;

fn main() {
    println!("{:?}", how_many_blocks(include_str!("input")));
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
    position: &mut (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> u32 {
    let mut distance_visited_twice = 0;
    let mut movement: (&mut i32, i32);

    match facing {
        Cardinal::North => movement = (&mut position.0, 1),
        Cardinal::East => movement = (&mut position.1, 1),
        Cardinal::South => movement = (&mut position.0, -1),
        Cardinal::West => movement = (&mut position.1, -1),
    };

    for _ in 1..distance {
        *movement.0 += movement.1;
        if !visited.insert(*position) {
            distance_visited_twice = manhattan_distance(position);
        }
    }
    distance_visited_twice
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
        let distance_visited_twice = advance(&facing, distance as i32, &mut position, &mut visited);
        if distance_visited_twice != 0 {
            result.0 = distance_visited_twice;
        }

        println!("{} {:?} {:?}", movement, position, visited);
    }
    result.1 = manhattan_distance(&position);
    result
}

#[test]
fn day01_examples() {
    assert_eq!(how_many_blocks("R2, L3").1, 5);
    assert_eq!(how_many_blocks("R2, R2, R2").1, 2);
    assert_eq!(how_many_blocks("R5, L5, R5, R3").1, 12);

    println!("DEBUGAX");
    assert_eq!(how_many_blocks("R8, R4, R4, R8").0, 4);
}
