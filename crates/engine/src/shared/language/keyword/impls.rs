use super::KeywordKind;
use phf::phf_map;

// ─────────────────────────────────────────────
// Longest keyword: "intersection" (12 bytes)
// Shortest keyword: "if", "do", etc. (2 bytes)
// ─────────────────────────────────────────────
const MAX_KEYWORD_LEN: usize = 12;

static KEYWORDS: phf::Map<&'static [u8], KeywordKind> = phf_map! {
	// len 3
	b"nil" => KeywordKind::NilValue,
	b"let" => KeywordKind::Variable,
	b"for" => KeywordKind::For,
	b"try" => KeywordKind::Try,
	b"use" => KeywordKind::Use,
	b"new" => KeywordKind::New,
	b"set" => KeywordKind::Set,
	b"get" => KeywordKind::Get,
	b"and" => KeywordKind::And,
	b"not" => KeywordKind::Not,
	b"xor" => KeywordKind::Xor,
	b"may" => KeywordKind::May,
	// len 4
	b"auto" => KeywordKind::AutoValue,
	b"none" => KeywordKind::NoneValue,
	b"bool" => KeywordKind::Boolean,
	b"else" => KeywordKind::Else,
	b"when" => KeywordKind::When,
	b"case" => KeywordKind::Case,
	b"loop" => KeywordKind::Loop,
	b"exit" => KeywordKind::Exit,
	b"join" => KeywordKind::Join,
	b"from" => KeywordKind::From,
	b"type" => KeywordKind::Type,
	b"with" => KeywordKind::With,
	b"enum" => KeywordKind::Enum,
	b"this" => KeywordKind::This,
	b"self" => KeywordKind::SelfScope,
	b"root" => KeywordKind::Root,
	b"here" => KeywordKind::Here,
	b"lazy" => KeywordKind::Lazy,
	b"only" => KeywordKind::Only,
	b"meta" => KeywordKind::Meta,
	// len 5
	b"const" => KeywordKind::Constant,
	b"alias" => KeywordKind::Alias,
	b"entry" => KeywordKind::Entry,
	b"match" => KeywordKind::Match,
	b"while" => KeywordKind::While,
	b"until" => KeywordKind::Until,
	b"break" => KeywordKind::Break,
	b"yield" => KeywordKind::Yield,
	b"async" => KeywordKind::Async,
	b"await" => KeywordKind::Await,
	b"spawn" => KeywordKind::Spawn,
	b"defer" => KeywordKind::Defer,
	b"catch" => KeywordKind::Catch,
	b"throw" => KeywordKind::Throw,
	b"where" => KeywordKind::Where,
	b"class" => KeywordKind::Class,
	b"union" => KeywordKind::Union,
	b"super" => KeywordKind::Super,
	b"local" => KeywordKind::Local,
	b"final" => KeywordKind::Final,
	b"unset" => KeywordKind::Unset,
	b"delta" => KeywordKind::Delta,
	b"event" => KeywordKind::Event,
	// len 6
	b"schema" => KeywordKind::Schema,
	b"switch" => KeywordKind::Switch,
	b"return" => KeywordKind::Return,
	b"cancel" => KeywordKind::Cancel,
	b"atomic" => KeywordKind::Atomic,
	b"import" => KeywordKind::Import,
	b"export" => KeywordKind::Export,
	b"struct" => KeywordKind::Struct,
	b"origin" => KeywordKind::Origin,
	b"parent" => KeywordKind::Parent,
	b"public" => KeywordKind::Public,
	b"global" => KeywordKind::Global,
	b"static" => KeywordKind::Static,
	b"strict" => KeywordKind::Strict,
	b"sorted" => KeywordKind::Sorted,
	b"unique" => KeywordKind::Unique,
	b"inline" => KeywordKind::Inline,
	b"packed" => KeywordKind::Packed,
	b"spread" => KeywordKind::Spread,
	b"filter" => KeywordKind::Filter,
	b"repeat" => KeywordKind::Repeat,
	b"marker" => KeywordKind::Marker,
	// len 7
	b"declare" => KeywordKind::Declare,
	b"default" => KeywordKind::Default,
	b"finally" => KeywordKind::Finally,
	b"include" => KeywordKind::Include,
	b"generic" => KeywordKind::Generic,
	b"extends" => KeywordKind::Extends,
	b"private" => KeywordKind::Private,
	b"virtual" => KeywordKind::Virtual,
	b"mutable" => KeywordKind::Mutable,
	b"trigger" => KeywordKind::Trigger,
	b"combine" => KeywordKind::Combine,
	b"flatten" => KeywordKind::Flatten,
	b"bitwise" => KeywordKind::Bitwise,
	b"provide" => KeywordKind::Provide,
	b"reflect" => KeywordKind::Reflect,
	b"section" => KeywordKind::SectionMaker,
	// len 8
	b"infinity" => KeywordKind::NumberInfinity,
	b"continue" => KeywordKind::Continue,
	b"deferred" => KeywordKind::Deferred,
	b"contains" => KeywordKind::Contains,
	b"callable" => KeywordKind::Callable,
	b"nullable" => KeywordKind::Nullable,
	b"internal" => KeywordKind::Internal,
	b"external" => KeywordKind::External,
	b"abstract" => KeywordKind::Abstract,
	b"override" => KeywordKind::Override,
	b"unsorted" => KeywordKind::Unsorted,
	b"reactive" => KeywordKind::Reactive,
	b"computed" => KeywordKind::Computed,
	b"generate" => KeywordKind::Generate,
	// len 9
	b"undefined" => KeywordKind::UndefinedValue,
	b"coroutine" => KeywordKind::Coroutine,
	b"interface" => KeywordKind::Interface,
	b"protected" => KeywordKind::Protected,
	b"immutable" => KeywordKind::Immutable,
	b"untrigger" => KeywordKind::Untrigger,
	b"enumerate" => KeywordKind::Enumerate,
	b"transform" => KeywordKind::Transform,
	b"transpose" => KeywordKind::Transpose,
	b"condition" => KeywordKind::Condition,
	b"attribute" => KeywordKind::Attribute,
	// len 10
	b"implements" => KeywordKind::Implements,
	b"capability" => KeywordKind::Capability,
	// len 12
	b"intersection" => KeywordKind::Intersection,
};

impl KeywordKind {
	pub fn from_bytes(slice: &[u8]) -> Option<Self> {
		let len = slice.len();

		// Fast reject: outside keyword length bounds
		if len < 2 || len > MAX_KEYWORD_LEN {
			return None;
		}

		// Direct comparison for 2-byte keywords — cheaper than hashing
		if len == 2 {
			return match (slice[0], slice[1]) {
				(b'i', b'f') => Some(Self::If),
				(b'd', b'o') => Some(Self::Do),
				(b'a', b's') => Some(Self::As),
				(b'i', b's') => Some(Self::Is),
				(b'b', b'e') => Some(Self::Be),
				(b'i', b'n') => Some(Self::In),
				(b'o', b'f') => Some(Self::Of),
				(b'o', b'n') => Some(Self::On),
				(b'o', b'r') => Some(Self::Or),
				_ => None,
			};
		}

		KEYWORDS.get(slice).copied()
	}
}
