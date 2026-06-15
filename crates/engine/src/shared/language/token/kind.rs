use super::super::{KeywordKind, OperatorKind, SyntaxKind};
// use crate::shared::UnitKind;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
	// ─────────────────────────────────────────────
	// Служебные
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
	SectionMaker,   // §  \\ Scope

	Underscore, // _  \\ Wildcard

	Expression, // \\ Some / Some * Soma + Some...

	// ─────────────────────────────────────────────
	// Literals and identifiers
	// ─────────────────────────────────────────────
	Identifier,             // name \\ Identifier
	Keyword(KeywordKind),   // let \\ Keyword
	Operator(OperatorKind), // + \\ Operator
	Syntax(SyntaxKind),     // ( \\ Syntax
	Number,                 // 123  \\ NumericLiteral
	String,                 // " "  \\ StringLiteral
	Placeholder,            // _    \\ Placeholder / PartialApply
	// Unit(UnitKind),

	// ─────────────────────────────────────────────
	// Comments
	// ─────────────────────────────────────────────
	LineComment,  // //   \\ LineComment
	BlockComment, // /* */\\ BlockComment
}
