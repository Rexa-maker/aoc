use std::fs;

pub struct Input {
    pub input: String,
}

impl Input {
    pub fn solve(self, solver: impl Fn(&str) -> String) -> String {
        solver(self.input.as_str())
    }
}

pub fn open(path: &str) -> Input {
    Input {
        input: fs::read_to_string(path).expect("Can't open file"),
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
