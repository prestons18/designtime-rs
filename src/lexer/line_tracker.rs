pub struct LineTracker {
    pub line: usize,
    pub column: usize,
}

impl LineTracker {
    pub fn new() -> Self {
        Self { line: 1, column: 0 }
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}
