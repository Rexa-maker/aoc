fn main() {
    let input = 312051;
    let steps = steps(&input);
    println!("{} -> {} steps", input, steps);
}

/**
 * The bottom right of each "loop" is a square of an odd number: 1, 9, 25, 49...
 * The size of the side of each "loop" is that same odd number: 1, 3, 5, 7...
 * The shortest distance in each "loop" is on a cardinal axis, and is the half of that odd number: 0, 1, 2, 3...
 * The longest distance in each "loop" is on the diagonal, and is that odd number - 1: 0, 2, 4, 6...
 *
 *
 * 37  36  35  34  33  32  31  
 * 38  17  16  15  14  13  30
 * 39  18   5   4   3  12  29
 * 40  19   6   1   2  11  28
 * 41  20   7   8   9  10  27
 * 42  21  22  23  24  25  26
 * 43  44  45  46  47  48  49  ...
 */
fn steps(block: &u32) -> u32 {
    if *block == 1 {
        return 0;
    }

    // Let's find the root of the bottom right number
    let mut root = 1;
    loop {
        if root * root >= *block {
            break;
        }
        root += 2;
    }
    let half_root = root / 2;

    // Let's find our distance to a cardinal axis
    // First, remove the previous odd square to only keep the outer loop
    let outer_loop = block - ((root - 2).pow(2));
    // Then, find out where on the side we are
    let position_on_side = outer_loop % (root - 1);
    // Finally, find the distance to the closest cardinal position
    // Effin'g i32 & u32 don't match well enough to do an abs
    let distance_to_cardinal = if half_root > position_on_side {
        half_root - position_on_side
    } else {
        position_on_side - half_root
    };

    // Return that distance plus the distance to cardinal
    distance_to_cardinal + half_root
}

#[test]
fn day03_examples() {
    let examples = [1, 12, 23, 1024];
    let solutions = [0, 3, 2, 31];

    for (i, block) in examples.iter().enumerate() {
        assert_eq!(steps(block), solutions[i]);
    }
}
