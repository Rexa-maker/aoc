fn main() {
    let input = include_str!("input");
    println!("{} {}", code1(input), code2(input));
}

/// This struct represents the digists 1 to 9 on a keypad:
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// It provides easy accessors to determine the currently selected digit,
/// and can move up, left, right, or down to the next digit.
struct Keypad {
    x: u8,
    y: u8,
}

impl Keypad {
    fn new() -> Keypad {
        Keypad { x: 1, y: 1 }
    }

    fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn down(&mut self) {
        if self.y < 2 {
            self.y += 1;
        }
    }

    fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn right(&mut self) {
        if self.x < 2 {
            self.x += 1;
        }
    }

    fn digit(&self) -> u8 {
        self.y * 3 + self.x + 1
    }
}

/// Same as Keypad, except now the pattern looks like this:
///     1
///   2 3 4
/// 5 6 7 8 9
///   A B C
///     D
struct Keypad2 {
    x: u8,
    y: u8,
}

impl Keypad2 {
    fn new() -> Keypad2 {
        Keypad2 { x: 0, y: 2 } // Start at 5
    }

    fn up(&mut self) {
        static CANT_GO_UP: [(u8, u8); 5] = [(0, 2), (1, 1), (2, 0), (3, 1), (4, 2)];
        if !CANT_GO_UP.contains(&(self.x, self.y)) {
            self.y -= 1;
        }
    }

    fn down(&mut self) {
        static CANT_GO_DOWN: [(u8, u8); 5] = [(0, 2), (1, 3), (2, 4), (3, 3), (4, 2)];
        if !CANT_GO_DOWN.contains(&(self.x, self.y)) {
            self.y += 1;
        }
    }

    fn left(&mut self) {
        static CANT_GO_LEFT: [(u8, u8); 5] = [(2, 0), (1, 1), (0, 2), (1, 3), (2, 4)];
        if !CANT_GO_LEFT.contains(&(self.x, self.y)) {
            self.x -= 1;
        }
    }

    fn right(&mut self) {
        static CANT_GO_RIGHT: [(u8, u8); 5] = [(2, 0), (3, 1), (4, 2), (3, 3), (2, 4)];
        if !CANT_GO_RIGHT.contains(&(self.x, self.y)) {
            self.x += 1;
        }
    }

    fn digit(&self) -> char {
        match (self.x, self.y) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 4) => 'C',
            (2, 4) => 'D',
            _ => panic!("Invalid keypad position ({}, {})", self.x, self.y),
        }
    }
}

fn code1(input: &str) -> u16 {
    let mut code = 0;
    let mut keypad = Keypad::new();

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => keypad.up(),
                'D' => keypad.down(),
                'L' => keypad.left(),
                'R' => keypad.right(),
                _ => panic!("Invalid input"),
            }
        }
        code = code * 10 + keypad.digit() as u16;
    }

    code
}

fn code2(input: &str) -> String {
    let mut code = String::new();
    let mut keypad = Keypad2::new();

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => keypad.up(),
                'D' => keypad.down(),
                'L' => keypad.left(),
                'R' => keypad.right(),
                _ => panic!("Invalid input"),
            }
        }
        code.push(keypad.digit());
    }

    code
}

#[test]
fn examples() {
    let instructions = "ULL
RRDDD
LURDL
UUUUD";
    assert_eq!(code1(instructions), 1985);
    assert_eq!(code2(instructions), "5DB3".to_string());
}
