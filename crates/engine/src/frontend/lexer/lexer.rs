use crate::shared::{
	KeywordKind, NumberKind, OperatorKind, SyntaxKind,
	Token, TokenFlags, TokenKind, Position,
};
use super::byte_class::{
	BYTE_CLASS,
	CLS_IDENT, CLS_DIGIT, CLS_SPACE, CLS_NEWLINE,
	CLS_OP, CLS_SYNTAX, CLS_UNICODE_OP, CLS_UNICODE_IDENT,
};
use wide::u8x16;

const EOF_CHAR: u8 = b'\0';
const CHUNK: usize = 16;

// ─────────────────────────────────────────────
// ASCII bitmask для идентификаторов
// Взят из легаси ASCII_CONTINUE — проверен в боевых условиях
// Включает: a-z A-Z 0-9 _ (без - т.к. нет UnitContext)
// ─────────────────────────────────────────────
const ASCII_IDENT_CONTINUE: u128 = 0x07ff_fffe_87ff_fffe_03ff_0000_0000_0000;

pub struct Lexer<'a> {
	src:           &'a [u8],
	start:         usize,
	current:       usize,
	position:      Position,
	start_position: Position,
	indent_stack:  Vec<usize>,
	at_line_start: bool,
	had_whitespace: bool,
	// переиспользуемый буфер — reset() не выделяет память
	tokens:        Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(src: &'a [u8]) -> Self {
		Self {
			src,
			start: 0,
			current: 0,
			position: Position::start(),
			start_position: Position::start(),
			indent_stack: vec![0],
			at_line_start: true,
			had_whitespace: false,
			tokens: Vec::with_capacity(src.len() / 4),
		}
	}

	/// Сброс без освобождения памяти Vec — ключевое для бенчмарка.
	pub fn reset(&mut self, src: &'a [u8]) {
		self.src           = src;
		self.start         = 0;
		self.current       = 0;
		self.position      = Position::start();
		self.start_position = Position::start();
		self.indent_stack.clear();
		self.indent_stack.push(0);
		self.at_line_start  = true;
		self.had_whitespace = false;
		self.tokens.clear(); // capacity сохраняется — нет аллокации
	}

	pub fn tokenize(&mut self) -> &[Token<'a>] {
		self.tokens.push(Token {
			kind: TokenKind::SOF,
			lexeme: None,
			position: self.position,
			flags: TokenFlags::empty(),
		});

		while !self.is_at_end() {
			if self.at_line_start {
				self.scan_indentation();
			}
			if !self.is_at_end() {
				self.start          = self.current;
				self.start_position = self.position;
				self.scan_token();
			}
		}

		// закрываем незакрытые отступы
		while self.indent_stack.len() > 1 {
			self.indent_stack.pop();
			let level = (self.indent_stack.len() - 1) as u8;
			self.push_raw(TokenKind::Dedent(level));
		}

		self.tokens.push(Token {
			kind: TokenKind::EOF,
			lexeme: None,
			position: self.position,
			flags: TokenFlags::empty(),
		});

		&self.tokens
	}

	// ─────────────────────────────────────────────
	// Основной диспетч — BYTE_CLASS вместо вложенных if/match
	// ─────────────────────────────────────────────

	fn scan_token(&mut self) {
		let b = self.advance();

		match BYTE_CLASS[b as usize] {
			CLS_SPACE => {
					let rest = &self.src[self.current..];
					let delta = rest.iter()
							.position(|&x| !matches!(x, b' ' | b'\t'))
							.unwrap_or(rest.len());
					self.current          += delta;
					self.position.offset  += delta as u32;
					self.had_whitespace    = true;
					self.start             = self.current;
					self.start_position    = self.position;
			}

			CLS_NEWLINE => {
				self.push_raw(TokenKind::Newline);
				self.at_line_start  = true;
				self.had_whitespace = false;
				self.start          = self.current;
				self.start_position = self.position;
			}

			CLS_IDENT | CLS_UNICODE_IDENT => {
				self.scan_identifier();
			}

			CLS_DIGIT => {
				self.scan_number();
			}

			CLS_SYNTAX => {
				match b {
					b'"' | b'\'' | b'`' => self.scan_string(b),
					_ => {
						if let Some(kind) = SyntaxKind::from_byte(b) {
							self.push_token(TokenKind::Syntax(kind));
						} else {
							self.push_token(TokenKind::Unknown);
						}
					}
				}
			}

			CLS_OP => {
				if b == b'_' {
					// подчёркивание — Wildcard если одиночный, иначе идентификатор
					let next = self.peek();
					if next == b'_'
						|| (next >= b'0' && next <= b'9')
						|| (next | 32 >= b'a' && next | 32 <= b'z')
						|| next >= 0x80
					{
						self.scan_identifier();
					} else {
						self.push_raw(TokenKind::Underscore);
					}
				} else {
					self.scan_operator(b);
				}
			}

			CLS_UNICODE_OP => {
				// advance() уже потребил первый байт Unicode последовательности.
				// match_operator ожидает срез начиная с первого байта — берём с self.start.
				// НЕ откатываем current: scan_operator скорректирует его через consume_count.
				self.scan_operator(b);
			}

			_ => {
				self.push_token(TokenKind::Unknown);
			}
		}
	}

	// ─────────────────────────────────────────────
	// Идентификаторы и ключевые слова
	// Взята механика легаси scan_identifier — for &b in итератор
	// для автовекторизации LLVM + bulk position update
	// ─────────────────────────────────────────────

	fn scan_identifier(&mut self) {
			let source     = self.src;
			let start_idx  = self.start;
			let scan_from  = self.current;
			let mut curr_idx = scan_from;

			if let Some(rest) = source.get(scan_from..) {
					for &b in rest {
							if b < 0x80 {
									if (ASCII_IDENT_CONTINUE & (1u128 << b)) != 0 {
											curr_idx += 1;
											continue;
									}
									break;
							}
							curr_idx += 1;
					}
			}

			let text_len = curr_idx - start_idx;
			self.current = curr_idx;

			// bulk position update — обновляем только offset
			self.position.offset = self.start_position.offset + text_len as u32;

			let kind = match KeywordKind::from_bytes(&source[start_idx..curr_idx]) {
					Some(kw) => TokenKind::Keyword(kw),
					None     => TokenKind::Identifier,
			};
			self.push_token(kind);
	}

	// ─────────────────────────────────────────────
	// Числа — используем NumberKind::from_bytes вместо старой логики radix
	// ─────────────────────────────────────────────

	fn scan_number(&mut self) {
			let tail = &self.src[self.start..];
			let kind = NumberKind::from_bytes(tail);

			match kind {
					NumberKind::Binary | NumberKind::Octal
					| NumberKind::Hexadecimal | NumberKind::Duotrigesimal => {
							if self.current < self.src.len() { self.current += 1; }
							let rest = &self.src[self.current..];
							let delta = rest.iter().position(|&b| {
									!b.is_ascii_alphanumeric() && b != b'_'
							}).unwrap_or(rest.len());
							self.current         += delta;
							self.position.offset  = self.start_position.offset + (self.current - self.start) as u32;
					}
					NumberKind::Decimal | NumberKind::Float => {
							let rest = &self.src[self.current..];
							let delta = rest.iter()
									.position(|&b| !b.is_ascii_digit() && b != b'_')
									.unwrap_or(rest.len());
							self.current += delta;

							let is_float = self.current < self.src.len()
									&& self.src[self.current] == b'.'
									&& self.src.get(self.current + 1).map_or(false, |b| b.is_ascii_digit());

							if is_float {
									self.current += 1;
									let rest = &self.src[self.current..];
									let delta = rest.iter()
											.position(|&b| !b.is_ascii_digit() && b != b'_')
											.unwrap_or(rest.len());
									self.current += delta;

									if self.current < self.src.len() && matches!(self.src[self.current], b'e' | b'E') {
											self.current += 1;
											if self.current < self.src.len() && matches!(self.src[self.current], b'+' | b'-') {
													self.current += 1;
											}
											let rest = &self.src[self.current..];
											let delta = rest.iter()
													.position(|&b| !b.is_ascii_digit())
													.unwrap_or(rest.len());
											self.current += delta;
									}
							}

							let len = self.current - self.start;
							self.position.offset = self.start_position.offset + len as u32;
							let actual_kind = if is_float { NumberKind::Float } else { NumberKind::Decimal };
							self.push_token(TokenKind::Number(actual_kind));
							return;
					}
			}
			self.push_token(TokenKind::Number(kind));
	}

	// ─────────────────────────────────────────────
	// Строки — SIMD поиск кавычки / эскейпа
	// ─────────────────────────────────────────────

	fn scan_string(&mut self, quote: u8) {
			// quote уже потреблён advance() в scan_token
			// открывающий токен синтаксиса
			let syntax = match quote {
					b'"'  => SyntaxKind::DoubleQuote,
					b'\'' => SyntaxKind::SingleQuote,
					_     => SyntaxKind::GraveAccent,
			};
			self.push_token(TokenKind::Syntax(syntax));

			// мультистрочная тройная кавычка только для "
			let is_multiline = quote == b'"'
					&& self.src.get(self.current).copied() == Some(b'"')
					&& self.src.get(self.current + 1).copied() == Some(b'"');
			if is_multiline {
					self.current += 2;
					self.position.offset += 2;
			}

			self.start          = self.current;
			self.start_position = self.position;
			self.scan_string_content(quote, is_multiline);
	}

	fn scan_string_content(&mut self, quote: u8, is_multiline: bool) {
		let content_start = self.current;

		loop {
			if self.is_at_end() { break; }

			let is_closing = if is_multiline {
				self.src.get(self.current).copied() == Some(b'"')
				&& self.src.get(self.current + 1).copied() == Some(b'"')
				&& self.src.get(self.current + 2).copied() == Some(b'"')
			} else {
				self.src.get(self.current).copied() == Some(quote)
			};

			if is_closing || (!is_multiline && self.src.get(self.current).copied() == Some(b'\n')) {
				break;
			}

			let b = self.advance();
			if b == b'\\' && !self.is_at_end() {
				self.advance();
			}
		}

		if self.is_at_end() || (!is_multiline && self.src.get(self.current).copied() == Some(b'\n')) {
			// незакрытая строка — Error токен
			if content_start != self.current {
				self.push_token(TokenKind::String);
			}
			return;
		}

		if content_start != self.current {
			self.push_token(TokenKind::String);
		}

		// закрывающая кавычка
		self.start          = self.current;
		self.start_position = self.position;
		let quote_len = if is_multiline { 3 } else { 1 };
		for _ in 0..quote_len { self.advance(); }

		let syntax = match quote {
			b'"'  => SyntaxKind::DoubleQuote,
			b'\'' => SyntaxKind::SingleQuote,
			_     => SyntaxKind::GraveAccent,
		};
		self.push_token(TokenKind::Syntax(syntax));
		self.start          = self.current;
		self.start_position = self.position;
	}

	// ─────────────────────────────────────────────
	// Операторы — OperatorKind::match_operator + SIMD для комментариев
	// ─────────────────────────────────────────────

	fn scan_operator(&mut self, _first_byte: u8) {
			let tail    = &self.src[self.start..];
			let op      = OperatorKind::match_operator(tail);
			let consume = op.consume_count as usize;

			if consume > 1 {
					let extra = consume - 1;
					self.current         += extra;
					self.position.offset += extra as u32;
			}
			self.position.offset = self.start_position.offset + consume.max(1) as u32;

			match op.token_kind {
					TokenKind::LineComment => {
							let end   = self.find_byte_simd(b'\n');
							let delta = end - self.current;
							self.current         += delta;
							self.position.offset += delta as u32;
							self.start          = self.current;
							self.start_position = self.position;
					}
					TokenKind::BlockComment => {
							while !self.is_at_end() {
									if self.src.get(self.current).copied() == Some(b'*')
											&& self.src.get(self.current + 1).copied() == Some(b'/')
									{
											self.advance();
											self.advance();
											break;
									}
									self.advance();
							}
							self.start          = self.current;
							self.start_position = self.position;
					}
					TokenKind::Unknown => {
							self.push_token(TokenKind::Unknown);
					}
					kind => {
							self.push_token(kind);
					}
			}
	}

	// ─────────────────────────────────────────────
	// Отступы — перенесено из legasi handle_indentation
	// ─────────────────────────────────────────────

	fn scan_indentation(&mut self) {
		// сбрасываем флаг сразу — scan_token не должен снова попасть сюда
		// на той же позиции
		self.at_line_start = false;

		let mut weight = 0usize;
		while !self.is_at_end() {
			match self.src[self.current] {
				b' ' => {
					weight += 1;
					self.advance();
				}
				b'\t' => {
					let last = *self.indent_stack.last().unwrap_or(&0);
					weight += if last == 0 { 4 } else { last };
					self.advance();
				}
				_ => break,
			}
		}

		// пустая строка или только пробелы до \n — не меняем отступ
		if self.is_at_end() || matches!(self.src[self.current], b'\n' | b'\r') {
			return;
		}

		let last = *self.indent_stack.last().unwrap_or(&0);
		if weight > last {
			self.indent_stack.push(weight);
			self.push_raw(TokenKind::Indent((self.indent_stack.len() - 1) as u8));
		} else if weight < last {
			while *self.indent_stack.last().unwrap_or(&0) > weight {
				self.indent_stack.pop();
				self.push_raw(TokenKind::Dedent((self.indent_stack.len() - 1) as u8));
			}
		}

		self.start          = self.current;
		self.start_position = self.position;
	}

	// ─────────────────────────────────────────────
	// SIMD: поиск байта (wide::u8x16)
	// ─────────────────────────────────────────────

	#[inline]
	fn find_byte_simd(&self, needle: u8) -> usize {
		let src   = self.src;
		let mut i = self.current;
		let splat = u8x16::splat(needle);

		while i + CHUNK <= src.len() {
			let mask = u8x16::from(&src[i..i + CHUNK])
				.simd_eq(splat)
				.to_bitmask();
			if mask != 0 { return i + mask.trailing_zeros() as usize; }
			i += CHUNK;
		}
		while i < src.len() {
			if src[i] == needle { return i; }
			i += 1;
		}
		src.len()
	}

	// ─────────────────────────────────────────────
	// Построение токенов
	// ─────────────────────────────────────────────

	/// Токен с lexeme = src[start..current].
	#[inline]
	fn push_token(&mut self, kind: TokenKind) {
		let lexeme  = Some(&self.src[self.start..self.current]);
		let mut f   = TokenFlags::empty();
		if self.at_line_start  { f.insert(TokenFlags::AT_LINE_START);           self.at_line_start  = false; }
		if self.had_whitespace { f.insert(TokenFlags::HAS_PRECEDING_WHITESPACE); self.had_whitespace = false; }
		self.tokens.push(Token { kind, lexeme, position: self.start_position, flags: f });
	}

	/// Токен без lexeme (Newline, Indent, Dedent, SOF, EOF).
	#[inline]
	fn push_raw(&mut self, kind: TokenKind) {
		self.tokens.push(Token {
			kind,
			lexeme:   None,
			position: self.start_position,
			flags:    TokenFlags::empty(),
		});
	}

	// ─────────────────────────────────────────────
	// Навигация по источнику
	// ─────────────────────────────────────────────

	/// Шагаем вперёд и обновляем позицию — как advance() в легаси.
	#[inline(always)]
	fn advance(&mut self) -> u8 {
			let b = self.src.get(self.current).copied().unwrap_or(EOF_CHAR);
			if b == EOF_CHAR { return EOF_CHAR; }
			self.current         += 1;
			self.position.offset += 1;
			b
	}

	#[inline(always)]
	fn peek(&self) -> u8 {
		self.src.get(self.current).copied().unwrap_or(EOF_CHAR)
	}

	#[inline(always)]
	fn is_at_end(&self) -> bool {
		self.current >= self.src.len()
	}
}
