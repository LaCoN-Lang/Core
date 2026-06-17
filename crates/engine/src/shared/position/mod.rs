use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub offset: u32,
}

impl Default for Position {
    fn default() -> Self {
        Self::start()
    }
}

impl Position {
    pub fn start() -> Self {
        Self { offset: 0 }
    }

    pub fn new(offset: u32) -> Self {
        Self { offset }
    }

    pub fn advance(&mut self, len: u32) {
        self.offset += len;
    }

    pub fn to_line_col(&self, source: &[u8]) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;
        for i in 0..self.offset as usize {
            if i >= source.len() { break; }
            if source[i] == b'\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset:{}", self.offset)
    }
}

impl std::ops::Add<u32> for Position {
    type Output = Position;
    fn add(self, rhs: u32) -> Self::Output {
        Position { offset: self.offset + rhs }
    }
}
