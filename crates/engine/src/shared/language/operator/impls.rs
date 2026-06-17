use super::OperatorKind;
use crate::shared::TokenKind;
use phf::phf_map;

// ─────────────────────────────────────────────
// Unicode operators — phf keyed by raw UTF-8 bytes
// ─────────────────────────────────────────────
static UNICODE_OPERATORS: phf::Map<&'static [u8], OperatorKind> = phf_map! {
	// 2-byte (0xC2 / 0xC3 prefix)
	b"\xC2\xB1" => OperatorKind::PlusMinus,                    // ±
	b"\xC3\x97" => OperatorKind::Multiplication,               // ×
	b"\xC3\xB7" => OperatorKind::Obelus,                       // ÷
	// 3-byte (0xE2 prefix)
	b"\xE2\x80\xA6" => OperatorKind::Ellipsis,                 // …
	b"\xE2\x88\x88" => OperatorKind::ElementOf,                // ∈
	b"\xE2\x88\x89" => OperatorKind::NotAnElementOf,           // ∉
	b"\xE2\x88\x8B" => OperatorKind::ContainsAsMember,         // ∋
	b"\xE2\x88\x8C" => OperatorKind::DoesNotContainsAsMember,  // ∌
	b"\xE2\x88\x94" => OperatorKind::DotPlusUni,               // ∔
	b"\xE2\x88\x98" => OperatorKind::Ring,                     // ∘
	b"\xE2\x88\xB8" => OperatorKind::Monus,                    // ∸
	b"\xE2\x89\x88" => OperatorKind::AlmostEqual,              // ≈
	b"\xE2\x89\xA0" => OperatorKind::NotEqualUni,              // ≠
	b"\xE2\x89\xA1" => OperatorKind::IdenticalTo,              // ≡
	b"\xE2\x89\xA3" => OperatorKind::StrictEqualUni,           // ≣
	b"\xE2\x89\xA4" => OperatorKind::LessEqualUni,             // ≤
	b"\xE2\x89\xA5" => OperatorKind::GreaterEqualUni,          // ≥
	b"\xE2\x8A\xBB" => OperatorKind::Xor,                      // ⊻
	b"\xE2\x8C\x88" => OperatorKind::CeilStart,                // ⌈
	b"\xE2\x8C\x89" => OperatorKind::CeilEnd,                  // ⌉
	b"\xE2\x8C\x8A" => OperatorKind::FloorStart,               // ⌊
	b"\xE2\x8C\x8B" => OperatorKind::FloorEnd,                 // ⌋
};
pub struct OpMatch {
    pub token_kind:    TokenKind,
    pub consume_count: u8,
}

impl OpMatch {
    #[inline]
    fn op(kind: OperatorKind, consumed: u8) -> Self {
        Self { token_kind: TokenKind::Operator(kind), consume_count: consumed }
    }

    #[inline]
    fn unknown() -> Self {
        Self { token_kind: TokenKind::Unknown, consume_count: 0 }
    }
}

impl OperatorKind {
    pub fn match_operator(tail: &[u8]) -> OpMatch {
        match tail {
            // ── ASCII operators ──────────────────────────────────────────
            [b'+', b'+', ..] => OpMatch::op(OperatorKind::PlusPlus, 2),
            [b'+', b'=', ..] => OpMatch::op(OperatorKind::PlusEqual, 2),
            [b'+', ..]       => OpMatch::op(OperatorKind::Plus, 1),

            [b'-', b'-', ..] => OpMatch::op(OperatorKind::MinusMinus, 2),
            [b'-', b'=', ..] => OpMatch::op(OperatorKind::MinusEqual, 2),
            [b'-', b'>', ..] => OpMatch::op(OperatorKind::DashGreater, 2),
            [b'-', ..]       => OpMatch::op(OperatorKind::Minus, 1),

            [b'*', b'*', b'=', ..] => OpMatch::op(OperatorKind::AsteriskAsteriskEqual, 3),
            [b'*', b'*', ..]       => OpMatch::op(OperatorKind::AsteriskAsterisk, 2),
            [b'*', b'=', ..]       => OpMatch::op(OperatorKind::AsteriskEqual, 2),
            [b'*', ..]             => OpMatch::op(OperatorKind::Asterisk, 1),

            [b'/', b'/', b'=', ..] => OpMatch::op(OperatorKind::SlashSlashEqual, 3),
            [b'/', b'/', ..]       => OpMatch::op(OperatorKind::SlashSlash, 2),
            [b'/', b'*', ..]       => OpMatch { token_kind: TokenKind::BlockComment, consume_count: 2 },
            [b'/', b'|', b'\\', ..] => OpMatch { token_kind: TokenKind::LineComment, consume_count: 3 },
            [b'/', b'=', ..]       => OpMatch::op(OperatorKind::SlashEqual, 2),
            [b'/', ..]             => OpMatch::op(OperatorKind::Slash, 1),

            [b'=', b'=', b'=', ..] => OpMatch::op(OperatorKind::EqualEqualEqual, 3),
            [b'=', b'=', b'>', ..] => OpMatch::op(OperatorKind::EqualEqualGreater, 3),
            [b'=', b'=', ..]       => OpMatch::op(OperatorKind::EqualEqual, 2),
            [b'=', b'>', ..]       => OpMatch::op(OperatorKind::EqualGreater, 2),
            [b'=', ..]             => OpMatch::op(OperatorKind::Equal, 1),

            [b'!', b'=', b'=', ..] => OpMatch::op(OperatorKind::NotEqualEqual, 3),
            [b'!', b'=', ..]       => OpMatch::op(OperatorKind::NotEqual, 2),
            [b'!', b':', ..]       => OpMatch::op(OperatorKind::NotColon, 2),
            [b'!', ..]             => OpMatch::op(OperatorKind::Exclamation, 1),

            [b'<', b'-', b'<', ..] => OpMatch::op(OperatorKind::LessMinusLess, 3),
            [b'<', b'-', b'>', ..] => OpMatch::op(OperatorKind::LessDashGreater, 3),
            [b'<', b'-', ..]       => OpMatch::op(OperatorKind::LessMinus, 2),
            [b'<', b'=', b'=', ..] => OpMatch::op(OperatorKind::LessEqualEqual, 3),
            [b'<', b'=', ..]       => OpMatch::op(OperatorKind::LessEqual, 2),
            [b'<', b'<', b'-', ..] => OpMatch::op(OperatorKind::LessLessDash, 3),
            [b'<', b'<', b'=', ..] => OpMatch::op(OperatorKind::LessLessEqual, 3),
            [b'<', b'<', b'<', ..] => OpMatch::op(OperatorKind::LessLessLess, 3),
            [b'<', b'<', ..]       => OpMatch::op(OperatorKind::LessLess, 2),
            [b'<', b'.', b'.', ..] => OpMatch::op(OperatorKind::LessDotDot, 3),
            [b'<', b'|', ..]       => OpMatch::op(OperatorKind::LessPipe, 2),
            [b'<', ..]             => OpMatch::op(OperatorKind::Less, 1),

            [b'>', b'>', b'-', ..] => OpMatch::op(OperatorKind::GreaterGreaterDash, 3),
            [b'>', b'>', b'=', ..] => OpMatch::op(OperatorKind::GreaterGreaterEqual, 3),
            [b'>', b'>', b'>', ..] => OpMatch::op(OperatorKind::GreaterGreaterGreater, 3),
            [b'>', b'>', ..]       => OpMatch::op(OperatorKind::GreaterGreater, 2),
            [b'>', b'=', ..]       => OpMatch::op(OperatorKind::GreaterEqual, 2),
            [b'>', b'-', b'>', ..] => OpMatch::op(OperatorKind::GreaterDashGreater, 3),
            [b'>', ..]             => OpMatch::op(OperatorKind::Greater, 1),

            [b'.', b'.', b'.', ..] => OpMatch::op(OperatorKind::DotDotDot, 3),
            [b'.', b'.', b'=', ..] => OpMatch::op(OperatorKind::DotDotEqual, 3),
            [b'.', b'.', b'<', ..] => OpMatch::op(OperatorKind::DotDotLess, 3),
            [b'.', b'.', ..]       => OpMatch::op(OperatorKind::DotDot, 2),
            [b'.', b'+', ..]       => OpMatch::op(OperatorKind::DotPlus, 2),
            [b'.', b'-', ..]       => OpMatch::op(OperatorKind::DotMinus, 2),
            [b'.', b'=', ..]       => OpMatch::op(OperatorKind::DotEqual, 2),
            [b'.', ..]             => OpMatch::op(OperatorKind::Dot, 1),

            [b'?', b'?', ..] => OpMatch::op(OperatorKind::QuestionQuestion, 2),
            [b'?', b'=', ..] => OpMatch::op(OperatorKind::QuestionEqual, 2),
            [b'?', b':', ..] => OpMatch::op(OperatorKind::QuestionColon, 2),
            [b'?', b'.', ..] => OpMatch::op(OperatorKind::QuestionDot, 2),
            [b'?', ..]       => OpMatch::op(OperatorKind::Question, 1),

            [b'%', b'%', ..] => OpMatch::op(OperatorKind::PercentPercent, 2),
            [b'%', b'=', ..] => OpMatch::op(OperatorKind::PercentEqual, 2),
            [b'%', ..]       => OpMatch::op(OperatorKind::Percent, 1),

            [b'^', b'^', ..] => OpMatch::op(OperatorKind::CircumflexCircumflex, 2),
            [b'^', b'=', ..] => OpMatch::op(OperatorKind::CircumflexEqual, 2),
            [b'^', ..]       => OpMatch::op(OperatorKind::Circumflex, 1),

            [b'&', b'&', b'=', ..] => OpMatch::op(OperatorKind::AmpersandAmpersandEqual, 3),
            [b'&', b'&', ..]       => OpMatch::op(OperatorKind::AmpersandAmpersand, 2),
            [b'&', b'=', ..]       => OpMatch::op(OperatorKind::AmpersandEqual, 2),
            [b'&', ..]             => OpMatch::op(OperatorKind::Ampersand, 1),

            [b'|', b'|', b'=', ..] => OpMatch::op(OperatorKind::PipePipeEqual, 3),
            [b'|', b'|', ..]       => OpMatch::op(OperatorKind::PipePipe, 2),
            [b'|', b'=', ..]       => OpMatch::op(OperatorKind::PipeEqual, 2),
            [b'|', b'>', ..]       => OpMatch::op(OperatorKind::PipeGreater, 2),
            [b'|', ..]             => OpMatch::op(OperatorKind::Pipe, 1),

            [b':', b':', ..] => OpMatch::op(OperatorKind::ColonColon, 2),
            [b':', b'=', ..] => OpMatch::op(OperatorKind::ColonEqual, 2),
            [b':', ..]       => OpMatch::op(OperatorKind::Colon, 1),

            [b'~', b'=', ..] => OpMatch::op(OperatorKind::TildeEqual, 2),
            [b'~', ..]       => OpMatch::op(OperatorKind::Tilde, 1),

            // ── Unicode operators ────────────────────────────────────────
            [0xC2 | 0xC3, ..] => match UNICODE_OPERATORS.get(tail.get(..2).unwrap_or(&[])) {
                Some(&kind) => OpMatch::op(kind, 2),
                None => OpMatch::unknown(),
            },
            [0xE2, ..] => match UNICODE_OPERATORS.get(tail.get(..3).unwrap_or(&[])) {
                Some(&kind) => OpMatch::op(kind, 3),
                None => OpMatch::unknown(),
            },

            _ => OpMatch::unknown(),
        }
    }
}
