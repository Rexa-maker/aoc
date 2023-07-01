fn main() {
    let input = include_str!("input");
    println!("{}", how_many_lights_after_n_steps(input, 100, 100));
}

fn parse_input(input: &str, grid_width: usize) -> Vec<Vec<bool>> {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();
    assert_eq!(grid.len(), grid_width);
    assert_eq!(grid[0].len(), grid_width);
    grid
}

fn step(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
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

fn how_many_lights_after_n_steps(input: &str, steps: usize, grid_width: usize) -> usize {
    let mut grid = parse_input(input, grid_width);
    for _ in 0..steps {
        step(&mut grid);
    }
    grid.iter().flatten().filter(|&&c| c).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_many_lights_after_n_steps() {
        assert_eq!(
            how_many_lights_after_n_steps(
                ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
                4,
                6
            ),
            4
        );
    }
}
