#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NumberKind {
	Decimal,				// 100; 0–9
	Float, 					// 1.0; 0–9.0–9
	Binary,					// 0b1100100; 0–1
	Octal,					// 0o144; 0–7
	Hexadecimal,		// 0x64; 0–9ABCDEF
	Duotrigesimal		// 0z34; 0–9ABCDEFGHKMNPQRSTUVWXYZ
}
