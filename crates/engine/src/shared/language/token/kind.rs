use super::super::{KeywordKind, OperatorKind, SyntaxKind, NumberKind};
// use crate::shared::UnitKind;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenKind {
	// ─────────────────────────────────────────────
	// Service
	// ─────────────────────────────────────────────
	Illegal,
	Invalid, // \\ InvalidToken
	Error,   // \\ LexicalError
	Unknown, // \\ UnknownToken
	SOF,     // \\ StartOfFile
	EOF,     // \\ EndOfFile

	// ─────────────────────────────────────────────
	// Layout / whitespace-sensitive синтаксис
	// ─────────────────────────────────────────────
	Newline,        // \n \\ LineBreak
	CarriageReturn, // \r \\ CarriageReturn
	Indent(u8),     // →  \\ IndentIncrease
	Dedent(u8),     // ←  \\ IndentDecrease

	Underscore, // _  \\ Wildcard

	// ─────────────────────────────────────────────
	// Literals and identifiers
	// ─────────────────────────────────────────────
	Identifier,             // name \\ Identifier
	Keyword(KeywordKind),   // let \\ Keyword
	Operator(OperatorKind), // + \\ Operator
	Syntax(SyntaxKind),     // ( \\ Syntax
	Number(NumberKind),                 // 123  \\ NumericLiteral
	String,                 // " "  \\ StringLiteral

	// ─────────────────────────────────────────────
	// Comments
	// ─────────────────────────────────────────────
	LineComment,  // /#   \\ LineComment
	BlockComment, // /* */\\ BlockComment
}
