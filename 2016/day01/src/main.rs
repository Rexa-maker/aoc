fn main() {
    println!("{}", how_many_blocks(include_str!("input")));
}

enum Cardinal {
    North,
    East,
    South,
    West,
}

fn advance(facing: &Cardinal, distance: i32, position: &mut (i32, i32)) {
    match facing {
        Cardinal::North => position.0 += distance,
        Cardinal::East => position.1 += distance,
        Cardinal::South => position.0 -= distance,
        Cardinal::West => position.1 -= distance,
    }
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

fn how_many_blocks(input: &str) -> u32 {
    let mut position: (i32, i32) = (0, 0);
    let mut facing = Cardinal::North;

    for movement in input.split(", ") {
        let mut chars = movement.chars();

        let rotation = chars.next().unwrap();
        let distance: u32 = chars.collect::<String>().parse().unwrap();
        turn(rotation, &mut facing);
        advance(&facing, distance as i32, &mut position);
    }

    (position.0.abs() + position.1.abs()) as u32
}

#[test]
fn day01_examples() {
    assert_eq!(how_many_blocks("R2, L3"), 5);
    assert_eq!(how_many_blocks("R2, R2, R2"), 2);
    assert_eq!(how_many_blocks("R5, L5, R5, R3"), 12);
}
