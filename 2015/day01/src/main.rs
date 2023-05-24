use utils::Input;

fn main() {
    println!("{}", Input::new(include_str!("input")).solve(solver));
}

fn solver(input: &str) -> String {
    what_floor(input).to_string()
}

fn what_floor(input: &str) -> i32 {
    let mut floor = 0;

    for a in input.chars() {
        match a {
            '(' => {
                floor = floor + 1;
            }
            ')' => {
                floor = floor - 1;
            }
            _ => {}
        }
    }

    floor
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
}
