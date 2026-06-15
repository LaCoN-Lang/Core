use super::TokenKind;
use crate::shared::Position;
use bitflags::bitflags;

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct TokenFlags: u8 {
		const AT_LINE_START = 0b0000_0001;
		const HAS_PRECEDING_WHITESPACE = 0b0000_0010;
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
	pub lexeme: Option<&'a [u8]>,
	pub position: Position,

	pub kind: TokenKind,
	pub flags: TokenFlags,
}
