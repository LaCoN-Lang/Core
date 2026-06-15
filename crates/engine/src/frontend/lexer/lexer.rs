use crate::shared::{KeywordKind, SyntaxKind, Token, TokenKind, NumberKind};

const ASCII_START: u128 = 0x07ff_fffe_07ff_fffe_0000_0000_0000_0000;
const ASCII_CONTINUE: u128 = 0x07ff_fffe_87ff_fffe_03ff_0000_0000_0000;
const EOF_CHAR: u8 = b'\0';

// SIMD / crate wide
// bytes lexing
