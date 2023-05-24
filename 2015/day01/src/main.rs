use utils::Input;

fn main() {
    println!("{}", Input::new(include_str!("input")).solve(solver));
}

fn solver(input: &str) -> String {
    format!("{:?} {:?}", what_floor(input), basement(input))
}

struct Solver<'a> {
    remainder: Option<&'a str>,
    current_floor: i32,
}

impl Solver<'_> {
    fn new<'a>(input: &'a str) -> Solver {
        Solver {
            remainder: Some(input),
            current_floor: 0,
        }
    }
}

impl Iterator for Solver<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        fn char_to_increment(c: char) -> i32 {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        }

        if let Some(remainder) = self.remainder {
            if remainder.len() == 1 {
                self.remainder = None;
            } else {
                self.remainder = Some(remainder.split_at(1).1);
            }
            self.current_floor += char_to_increment(remainder.chars().next().unwrap());
            Some(self.current_floor)
        } else {
            None
        }
    }
}

fn what_floor(input: &str) -> i32 {
    *Solver::new(input).collect::<Vec<i32>>().last().unwrap()
}

fn basement(input: &str) -> u32 {
    let mut basement: u32 = 0;
    for next in Solver::new(input) {
        if next == -1 {
            return basement + 1;
        } else {
            basement += 1;
        }
    }
    0
}

#[test]
fn examples() {
    assert_eq!(0, what_floor("(())"));
    assert_eq!(0, what_floor("()()"));

    assert_eq!(3, what_floor("((("));
    assert_eq!(3, what_floor("(()(()("));
    assert_eq!(3, what_floor("))((((("));

    assert_eq!(-1, what_floor("())"));
    assert_eq!(-1, what_floor("))("));

    assert_eq!(-3, what_floor(")))"));
    assert_eq!(-3, what_floor(")())())"));

    assert_eq!(1, basement(")"));
    assert_eq!(5, basement("()())"));
}
