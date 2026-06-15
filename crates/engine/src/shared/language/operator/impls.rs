use super::OperatorKind;
use phf::phf_map;

// ─────────────────────────────────────────────
// Unicode operators — phf keyed by raw UTF-8 bytes
// All unicode operators are 2 or 3 bytes in UTF-8
// First bytes are always 0xC2, 0xC3, or 0xE2
// ─────────────────────────────────────────────
static UNICODE_OPERATORS: phf::Map<&'static [u8], OperatorKind> = phf_map! {
    // 2-byte unicode (0xC2 / 0xC3 prefix)
    b"\xC2\xB1" => OperatorKind::PlusMinus,         // ±
    b"\xC3\x97" => OperatorKind::Multiplication,     // ×
    b"\xC3\xB7" => OperatorKind::Obelus,             // ÷

    // 3-byte unicode (0xE2 prefix)
    b"\xE2\x80\xA6" => OperatorKind::Ellipsis,                   // …
    b"\xE2\x88\x88" => OperatorKind::ElementOf,                   // ∈
    b"\xE2\x88\x89" => OperatorKind::NotAnElementOf,              // ∉
    b"\xE2\x88\x8B" => OperatorKind::ContainsAsMember,            // ∋
    b"\xE2\x88\x8C" => OperatorKind::DoesNotContainsAsMember,     // ∌
    b"\xE2\x88\x94" => OperatorKind::DotPlusUni,                  // ∔
    b"\xE2\x88\x98" => OperatorKind::Ring,                        // ∘
    b"\xE2\x88\xB8" => OperatorKind::Monus,                       // ∸
    b"\xE2\x89\x88" => OperatorKind::AlmostEqual,                 // ≈
    b"\xE2\x89\xA0" => OperatorKind::NotEqualUni,                 // ≠
    b"\xE2\x89\xA1" => OperatorKind::IdenticalTo,                 // ≡
    b"\xE2\x89\xA3" => OperatorKind::StrictEqualUni,              // ≣
    b"\xE2\x89\xA4" => OperatorKind::LessEqualUni,                // ≤
    b"\xE2\x89\xA5" => OperatorKind::GreaterEqualUni,             // ≥
    b"\xE2\x8A\xBB" => OperatorKind::Xor,                        // ⊻
    b"\xE2\x8C\x88" => OperatorKind::CeilStart,                   // ⌈
    b"\xE2\x8C\x89" => OperatorKind::CeilEnd,                     // ⌉
    b"\xE2\x8C\x8A" => OperatorKind::FloorStart,                  // ⌊
    b"\xE2\x8C\x8B" => OperatorKind::FloorEnd,                    // ⌋
};

pub struct OpMatch {
    pub token_kind: TokenKind,
    pub consume_count: usize,
}

impl OpMatch {
    #[inline]
    fn op(kind: OperatorKind, consumed: usize) -> Self {
        Self {
            token_kind: TokenKind::Operator(kind),
            consume_count: consumed,
        }
    }

    #[inline]
    fn unknown() -> Self {
        Self {
            token_kind: TokenKind::Unknown,
            consume_count: 0,
        }
    }
}

impl OperatorKind {
    pub fn match_operator(tail: &[u8]) -> OpMatch {
        if tail.is_empty() {
            return OpMatch::unknown();
        }

        let c0 = tail[0];
        let c1 = tail.get(1).copied();
        let c2 = tail.get(2).copied();

        // ─────────────────────────────────────────────
        // Unicode operators — first byte is always >= 0xC2
        // ─────────────────────────────────────────────
        if c0 >= 0xC2 {
            // 2-byte unicode: 0xC2 or 0xC3 prefix
            if c0 == 0xC2 || c0 == 0xC3 {
                if tail.len() >= 2 {
                    if let Some(&kind) = UNICODE_OPERATORS.get(&tail[..2]) {
                        return OpMatch::op(kind, 2);
                    }
                }
            }
            // 3-byte unicode: 0xE2 prefix
            else if c0 == 0xE2 {
                if tail.len() >= 3 {
                    if let Some(&kind) = UNICODE_OPERATORS.get(&tail[..3]) {
                        return OpMatch::op(kind, 3);
                    }
                }
            }
            return OpMatch::unknown();
        }

        // ─────────────────────────────────────────────
        // ASCII operators — inline trie, max 3 chars deep
        // ─────────────────────────────────────────────
        match c0 {
            b'+' => match c1 {
                Some(b'+') => OpMatch::op(OperatorKind::PlusPlus, 2),
                Some(b'=') => OpMatch::op(OperatorKind::PlusEqual, 2),
                _           => OpMatch::op(OperatorKind::Plus, 1),
            },

            b'-' => match c1 {
                Some(b'-') => OpMatch::op(OperatorKind::MinusMinus, 2),
                Some(b'=') => OpMatch::op(OperatorKind::MinusEqual, 2),
                Some(b'>') => OpMatch::op(OperatorKind::DashGreater, 2),
                _           => OpMatch::op(OperatorKind::Minus, 1),
            },

            b'*' => match c1 {
                Some(b'*') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::AsteriskAsteriskEqual, 3),
                    _           => OpMatch::op(OperatorKind::AsteriskAsterisk, 2),
                },
                Some(b'=') => OpMatch::op(OperatorKind::AsteriskEqual, 2),
                _           => OpMatch::op(OperatorKind::Asterisk, 1),
            },

            b'/' => match c1 {
                Some(b'/') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::SlashSlashEqual, 3),
                    _           => OpMatch::op(OperatorKind::SlashSlash, 2),
                },
                Some(b'*') => OpMatch {
                    token_kind: TokenKind::BlockComment,
                    consume_count: 2,
                },
                Some(b'=') => OpMatch::op(OperatorKind::SlashEqual, 2),
                Some(b'|') if c2 == Some(b'\\') => OpMatch {
                    token_kind: TokenKind::LineComment,
                    consume_count: 3,
                },
                _ => OpMatch::op(OperatorKind::Slash, 1),
            },

            b'=' => match c1 {
                Some(b'=') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::EqualEqualEqual, 3),
                    _           => OpMatch::op(OperatorKind::EqualEqual, 2),
                },
                Some(b'>') => OpMatch::op(OperatorKind::EqualGreater, 2),
                _           => OpMatch::op(OperatorKind::Equal, 1),
            },

            b'!' => match c1 {
                Some(b'=') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::NotEqualEqual, 3),
                    _           => OpMatch::op(OperatorKind::NotEqual, 2),
                },
                Some(b':') => OpMatch::op(OperatorKind::NotColon, 2),
                _           => OpMatch::op(OperatorKind::Exclamation, 1),
            },

            b'<' => match c1 {
                Some(b'-') => match c2 {
                    Some(b'<') => OpMatch::op(OperatorKind::LessMinusLess, 3),
                    _           => OpMatch::op(OperatorKind::LessMinus, 2),
                },
                Some(b'=') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::LessEqualEqual, 3),
                    _           => OpMatch::op(OperatorKind::LessEqual, 2),
                },
                Some(b'<') => match c2 {
                    Some(b'-') => OpMatch::op(OperatorKind::LessLessDash, 3),
                    Some(b'=') => OpMatch::op(OperatorKind::LessLessEqual, 3),
                    Some(b'<') => OpMatch::op(OperatorKind::LessLessLess, 3),
                    _           => OpMatch::op(OperatorKind::LessLess, 2),
                },
                Some(b'.') => match c2 {
                    Some(b'.') => OpMatch::op(OperatorKind::LessDotDot, 3),
                    _           => OpMatch::op(OperatorKind::Less, 1),
                },
                Some(b'|') => OpMatch::op(OperatorKind::LessPipe, 2),
                Some(b'~') => match c2 {
                    Some(b'>') => OpMatch::op(OperatorKind::LessDashGreater, 3),
                    _           => OpMatch::op(OperatorKind::Less, 1),
                },
                _           => OpMatch::op(OperatorKind::Less, 1),
            },

            b'>' => match c1 {
                Some(b'=') => OpMatch::op(OperatorKind::GreaterEqual, 2),
                Some(b'>') => match c2 {
                    Some(b'-') => OpMatch::op(OperatorKind::GreaterGreaterDash, 3),
                    Some(b'=') => OpMatch::op(OperatorKind::GreaterGreaterEqual, 3),
                    Some(b'>') => OpMatch::op(OperatorKind::GreaterGreaterGreater, 3),
                    _           => OpMatch::op(OperatorKind::GreaterGreater, 2),
                },
                Some(b'-') => match c2 {
                    Some(b'>') => OpMatch::op(OperatorKind::GreaterDashGreater, 3),
                    _           => OpMatch::op(OperatorKind::Greater, 1),
                },
                _           => OpMatch::op(OperatorKind::Greater, 1),
            },

            b'.' => match c1 {
                Some(b'.') => match c2 {
                    Some(b'.') => OpMatch::op(OperatorKind::DotDotDot, 3),
                    Some(b'=') => OpMatch::op(OperatorKind::DotDotEqual, 3),
                    Some(b'<') => OpMatch::op(OperatorKind::DotDotLess, 3),
                    _           => OpMatch::op(OperatorKind::DotDot, 2),
                },
                Some(b'+') => OpMatch::op(OperatorKind::DotPlus, 2),
                Some(b'-') => OpMatch::op(OperatorKind::DotMinus, 2),
                Some(b'=') => OpMatch::op(OperatorKind::DotEqual, 2),
                _           => OpMatch::op(OperatorKind::Dot, 1),
            },

            b'?' => match c1 {
                Some(b'?') => OpMatch::op(OperatorKind::QuestionQuestion, 2),
                Some(b'=') => OpMatch::op(OperatorKind::QuestionEqual, 2),
                Some(b':') => OpMatch::op(OperatorKind::QuestionColon, 2),
                Some(b'.') => OpMatch::op(OperatorKind::QuestionDot, 2),
                _           => OpMatch::op(OperatorKind::Question, 1),
            },

            b'%' => match c1 {
                Some(b'%') => OpMatch::op(OperatorKind::PercentPercent, 2),
                Some(b'=') => OpMatch::op(OperatorKind::PercentEqual, 2),
                _           => OpMatch::op(OperatorKind::Percent, 1),
            },

            b'^' => match c1 {
                Some(b'^') => OpMatch::op(OperatorKind::CircumflexCircumflex, 2),
                Some(b'=') => OpMatch::op(OperatorKind::CircumflexEqual, 2),
                _           => OpMatch::op(OperatorKind::Circumflex, 1),
            },

            b'&' => match c1 {
                Some(b'&') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::AmpersandAmpersandEqual, 3),
                    _           => OpMatch::op(OperatorKind::AmpersandAmpersand, 2),
                },
                Some(b'=') => OpMatch::op(OperatorKind::AmpersandEqual, 2),
                _           => OpMatch::op(OperatorKind::Ampersand, 1),
            },

            b'|' => match c1 {
                Some(b'|') => match c2 {
                    Some(b'=') => OpMatch::op(OperatorKind::PipePipeEqual, 3),
                    _           => OpMatch::op(OperatorKind::PipePipe, 2),
                },
                Some(b'=') => OpMatch::op(OperatorKind::PipeEqual, 2),
                Some(b'>') => OpMatch::op(OperatorKind::PipeGreater, 2),
                _           => OpMatch::op(OperatorKind::Pipe, 1),
            },

            b':' => match c1 {
                Some(b':') => OpMatch::op(OperatorKind::ColonColon, 2),
                Some(b'=') => OpMatch::op(OperatorKind::ColonEqual, 2),
                _           => OpMatch::op(OperatorKind::Colon, 1),
            },

            b'~' => match c1 {
                Some(b'=') => OpMatch::op(OperatorKind::TildeEqual, 2),
                _           => OpMatch::op(OperatorKind::Tilde, 1),
            },

            b'_' => OpMatch {
                token_kind: TokenKind::Underscore,
                consume_count: 1,
            },

            _ => OpMatch::unknown(),
        }
    }
}
