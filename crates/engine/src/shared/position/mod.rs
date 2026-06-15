use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	pub line:   u32,
	pub column: u32,
	pub offset: u32,
}

impl Default for Position {
	fn default() -> Self {
		Self::start()
	}
}

impl Position {
	pub fn start() -> Self {
		Self { line: 1, column: 1, offset: 0 }
	}

	pub fn new(line: u32, column: u32, offset: u32) -> Self {
		Self { line, column, offset }
	}

	// For ASCII and known-length UTF-8 sequences from the lexer
	pub fn advance_byte(&mut self, byte: u8) {
		self.offset += 1;
		if byte == b'\n' {
			self.line  += 1;
			self.column = 1;
		} else {
			self.column += 1;
		}
	}

	// For multi-byte UTF-8 sequences — lexer already knows the length
	pub fn advance_utf8(&mut self, byte_len: u32) {
		self.offset += byte_len;
		self.column += 1;
	}

	// Advance over a newline sequence specifically (\r\n or \n)
	pub fn advance_newline(&mut self, byte_len: u32) {
		self.offset += byte_len;
		self.line   += 1;
		self.column  = 1;
	}

	pub fn shifted_byte(&self, byte: u8) -> Self {
		let mut pos = *self;
		pos.advance_byte(byte);
		pos
	}

	pub fn shifted_utf8(&self, byte_len: u32) -> Self {
		let mut pos = *self;
		pos.advance_utf8(byte_len);
		pos
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}:{}", self.line, self.column, self.offset)
	}
}

impl std::ops::Add<u32> for Position {
	type Output = Position;

	fn add(self, rhs: u32) -> Self::Output {
		Position {
			line:   self.line,
			column: self.column + rhs,
			offset: self.offset + rhs,
		}
	}
}
