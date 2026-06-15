pub const CLS_NONE:  u8 = 0;
pub const CLS_SYNTAX: u8 = 1;
pub const CLS_IDENT: u8 = 2; // a-z A-Z _
pub const CLS_DIGIT: u8 = 3; // 0-9
pub const CLS_SPACE: u8 = 4; // ' ' \t
pub const CLS_NEWLINE: u8 = 5; // \n \r
pub const CLS_OP:    u8 = 6; // + - * / = ! < > . ? % ^ & | : ~
pub const CLS_UNICODE_OP:    u8 = 7; // ± ÷ ±
pub const CLS_UNICODE_IDENT: u8 = 8; // А-Я а-я

const UTF8_C2: usize = 0xC2; // ±
const UTF8_C3: usize = 0xC3; // × ÷
const UTF8_E2: usize = 0xE2; // … ∈ ≠ ⊻ ⌊ и др.

const UTF8_D0: usize = 0xD0; // А-Я а-п Ё
const UTF8_D1: usize = 0xD1; // р-я ё

pub static BYTE_CLASS: [u8; 256] = {
    let mut t = [CLS_NONE; 256];

    // digits
    let mut i = b'0';
    while i <= b'9' { t[i as usize] = CLS_DIGIT; i += 1; }
    // lowercase
    let mut i = b'a';
    while i <= b'z' { t[i as usize] = CLS_IDENT; i += 1; }
    // uppercase
    let mut i = b'A';
    while i <= b'Z' { t[i as usize] = CLS_IDENT; i += 1; }
    // underscore
    t[b'_' as usize] = CLS_IDENT;
    // whitespace
    t[b' ' as usize]  = CLS_SPACE;
    t[b'\t' as usize] = CLS_SPACE;
    // newlines
    t[b'\n' as usize] = CLS_NEWLINE;
    t[b'\r' as usize] = CLS_NEWLINE;
		// syntax
		t[b'(' as usize] = CLS_SYNTAX;
		t[b')' as usize] = CLS_SYNTAX;
		t[b'[' as usize] = CLS_SYNTAX;
		t[b']' as usize] = CLS_SYNTAX;
		t[b'{' as usize] = CLS_SYNTAX;
		t[b'}' as usize] = CLS_SYNTAX;
		t[b'\'' as usize] = CLS_SYNTAX;
		t[b'"' as usize] = CLS_SYNTAX;
		t[b'`' as usize] = CLS_SYNTAX;
		t[b';' as usize] = CLS_SYNTAX;
		t[b',' as usize] = CLS_SYNTAX;
		t[b'\\' as usize] = CLS_SYNTAX;
		t[b'$' as usize] = CLS_SYNTAX;
		t[b'@' as usize] = CLS_SYNTAX;
		t[b'#' as usize] = CLS_SYNTAX;
    // operators
    t[b'+' as usize] = CLS_OP;
    t[b'-' as usize] = CLS_OP;
    t[b'*' as usize] = CLS_OP;
    t[b'/' as usize] = CLS_OP;
    t[b'=' as usize] = CLS_OP;
    t[b'!' as usize] = CLS_OP;
    t[b'<' as usize] = CLS_OP;
    t[b'>' as usize] = CLS_OP;
    t[b'.' as usize] = CLS_OP;
    t[b'?' as usize] = CLS_OP;
    t[b'%' as usize] = CLS_OP;
    t[b'^' as usize] = CLS_OP;
    t[b'&' as usize] = CLS_OP;
    t[b'|' as usize] = CLS_OP;
    t[b':' as usize] = CLS_OP;
    t[b'~' as usize] = CLS_OP;
    // unicode lead bytes
    let mut i: usize = 0x80;
    while i < 256 { t[i] = CLS_UNICODE_IDENT; i += 1; }
		// unicode operators
		t[UTF8_C2] = CLS_UNICODE_OP;
		t[UTF8_C3] = CLS_UNICODE_OP;
		t[UTF8_E2] = CLS_UNICODE_OP;
		t[UTF8_D0] = CLS_UNICODE_IDENT;
		t[UTF8_D1] = CLS_UNICODE_IDENT;
    t
};
