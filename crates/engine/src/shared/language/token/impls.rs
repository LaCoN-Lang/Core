use super::super::KeywordKind;
use super::{Token, TokenFlags, TokenKind};
use crate::shared::{Position, UnitKind};
use std::fmt;

impl<'a> Token<'a> {
	pub fn new(kind: TokenKind, is_at_line_start: bool, has_whitespace: bool, lexeme: Option<&'a [u8]>, position: Position) -> Self {
		let mut flags = TokenFlags::empty();
		if is_at_line_start {
			flags.insert(TokenFlags::AT_LINE_START);
		}
		if has_whitespace {
			flags.insert(TokenFlags::HAS_PRECEDING_WHITESPACE);
		}

		Self { kind, lexeme, position, flags }
	}

	pub fn bare(kind: TokenKind, position: Position) -> Self {
		Self {
			kind,
			lexeme: None,
			position,
			flags: TokenFlags::empty(),
		}
	}

	pub fn error(message: &'a [u8], position: Position) -> Self {
		Self {
			kind: TokenKind::Error,
			lexeme: Some(&message),
			position,
			flags: TokenFlags::empty(),
		}
	}
}

impl TokenKind {
	pub fn is_unit(&self) -> bool {
		match self {
			TokenKind::Unit(_) => true,
			_ => false,
		}
	}

	pub fn unit(&self) -> Option<UnitKind> {
		match self {
			TokenKind::Unit(s) => Some(*s),
			_ => None,
		}
	}

	pub fn is_keyword(&self) -> bool {
		match self {
			TokenKind::Keyword(_) => true,
			_ => false,
		}
	}

	pub fn keyword(&self) -> Option<KeywordKind> {
		match self {
			TokenKind::Keyword(k) => Some(*k),
			_ => None,
		}
	}
}

impl<'a> fmt::Display for Token<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{:?}", self.kind)?;

		if self.flags.contains(TokenFlags::AT_LINE_START) {
			f.write_str(" [SOL]")?;
		}
		if self.flags.contains(TokenFlags::HAS_PRECEDING_WHITESPACE) {
			f.write_str(" [WS]")?;
		}
		f.write_str("] ")?;

		// write!(f, "'{}'", self.lexeme)?;

		if let Some(literal) = &self.lexeme {
			write!(f, " (value: {:?})", literal)?;
		}

		write!(f, " at {}", self.position)
	}
}

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
