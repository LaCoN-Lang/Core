use super::NumberKind;

impl NumberKind {
	pub fn from_bytes(bytes: &[u8]) -> Self {
			if bytes.len() < 2 || bytes[0] != b'0' {
				return NumberKind::Decimal;
			}
			match bytes[1] {
				b'b' | b'B' => NumberKind::Binary,
				b'o' | b'O' => NumberKind::Octal,
				b'x' | b'X' => NumberKind::Hexadecimal,
				b'z' | b'Z' => NumberKind::Duotrigesimal,
				_ => NumberKind::Decimal,
			}
	}
}
