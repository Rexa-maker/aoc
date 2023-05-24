pub struct Input {
    pub input: String,
}

impl Input {
    pub fn new(data: &str) -> Input {
        Input {
            input: data.to_string(),
        }
    }

    pub fn solve(self, solver: impl Fn(&str) -> String) -> String {
        solver(self.input.as_str())
    }
}
