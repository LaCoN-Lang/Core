#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SyntaxKind {
	LeftParenthesis,  // (  \\ GroupStart
	RightParenthesis, // )  \\ GroupEnd
	LeftBracket,      // [  \\ IndexStart
	RightBracket,     // ]  \\ IndexEnd
	LeftBrace,        // {  \\ BlockStart
	RightBrace,       // }  \\ BlockEnd

	SingleQuote, // '
	DoubleQuote, // "
	GraveAccent, // `

	Semicolon, // ;
	Comma,     // ,

	Backslash, // \  \\ Escape or Difference

	Dollar, // $
	At,     // @
	Hash,   // #
}
