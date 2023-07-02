fn main() {
    let input = include_str!("input");
    println!(
        "{} {}",
        how_many_lights_after_n_steps(input, 100, 100, false),
        how_many_lights_after_n_steps(input, 100, 100, true)
    );
}

fn parse_input(input: &str, grid_width: usize, stuck_corners: bool) -> Vec<Vec<bool>> {
    let mut grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();
    assert_eq!(grid.len(), grid_width);
    assert_eq!(grid[0].len(), grid_width);

    if stuck_corners {
        grid[0][0] = true;
        grid[0][grid_width - 1] = true;
        grid[grid_width - 1][0] = true;
        grid[grid_width - 1][grid_width - 1] = true;
    }

    grid
}

fn step(grid: &mut Vec<Vec<bool>>, stuck_corners: bool) {
    let mut new_grid = grid.clone();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if stuck_corners && (x == 0 || x == grid.len() - 1) && (y == 0 || y == grid.len() - 1) {
                continue;
            }

            let mut neighbors_on = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || ny < 0 {
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;
                    if nx >= grid[y].len() || ny >= grid.len() {
                        continue;
                    }
                    if grid[ny][nx] {
                        neighbors_on += 1;
                    }
                }
            }
            if cell && (neighbors_on < 2 || neighbors_on > 3) {
                new_grid[y][x] = false;
            } else if !cell && neighbors_on == 3 {
                new_grid[y][x] = true;
            }
        }
    }

    *grid = new_grid;
}

fn how_many_lights_after_n_steps(
    input: &str,
    steps: usize,
    grid_width: usize,
    stuck_corners: bool,
) -> usize {
    let mut grid = parse_input(input, grid_width, stuck_corners);
    for _ in 0..steps {
        step(&mut grid, stuck_corners);
    }
    grid.iter().flatten().filter(|&&c| c).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_many_lights_after_n_steps() {
        static INPUT: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(how_many_lights_after_n_steps(INPUT, 4, 6, false), 4);
        assert_eq!(how_many_lights_after_n_steps(INPUT, 5, 6, true), 17);
    }
}
