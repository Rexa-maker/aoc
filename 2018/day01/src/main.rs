fn main() {
    let input = include_str!("input");
    println!("{}", frequency(input));
}

fn frequency(input: &str) -> i32 {
    input.lines().map(|val| {val.parse().unwrap()}).fold(0, |tally, x: i32| tally + x)
}

#[test]
fn examples() {
    assert_eq!(frequency("+1\n-2\n+3\n+1"), 3);
    assert_eq!(frequency("+1\n+1\n+1"), 3);
    assert_eq!(frequency("+1\n+1\n-2"), 0);
    assert_eq!(frequency("-1\n-2\n-3"), -6);
}
