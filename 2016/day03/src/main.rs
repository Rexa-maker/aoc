fn main() {
    let input = include_str!("input");
    println!(
        "{} {}",
        possible_triangles_by_line(input),
        possible_triangles_by_column(input)
    );
}

fn possible_triangles_by_line(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut sides = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            sides.sort();
            sides
        })
        .filter(|sides| sides[0] + sides[1] > sides[2])
        .count()
}

fn possible_triangles_by_column(input: &str) -> usize {
    let mut count = 0;
    let mut sides = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    while !sides.is_empty() {
        for i in 0..3 {
            let mut triangle = Vec::new();
            triangle.push(sides[0][i]);
            triangle.push(sides[1][i]);
            triangle.push(sides[2][i]);
            triangle.sort();
            if triangle[0] + triangle[1] > triangle[2] {
                count += 1;
            }
        }
        sides = sides[3..].to_vec();
    }

    count
}

#[test]
fn examples() {
    assert_eq!(possible_triangles_by_line("5 10 25"), 0);
    let input = "5  2 8
                       10 3 9
                       25 4 11";
    assert_eq!(possible_triangles_by_column(input), 2);
}
