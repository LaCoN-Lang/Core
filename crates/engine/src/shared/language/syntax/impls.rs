use super::SyntaxKind;

impl SyntaxKind {
	pub fn from_byte(byte: u8) -> Option<Self> {
		match byte {
			b'(' => Some(Self::LeftParenthesis),
			b')' => Some(Self::RightParenthesis),
			b'[' => Some(Self::LeftBracket),
			b']' => Some(Self::RightBracket),
			b'{' => Some(Self::LeftBrace),
			b'}' => Some(Self::RightBrace),
			b'\'' => Some(Self::SingleQuote),
			b'"'  => Some(Self::DoubleQuote),
			b'`'  => Some(Self::GraveAccent),
			b';'  => Some(Self::Semicolon),
			b','  => Some(Self::Comma),
			b'\\' => Some(Self::Backslash),
			b'$'  => Some(Self::Dollar),
			b'@'  => Some(Self::At),
			b'#'  => Some(Self::Hash),
			_     => None,
		}
	}
}
