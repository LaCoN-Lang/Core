#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeywordKind {
    // ─────────────────────────────────────────────
    // Values
    // ─────────────────────────────────────────────
    AutoValue,      // auto
    UndefinedValue, // undefined
    NoneValue,      // none
    NilValue,       // nil
    NumberInfinity, // infinity
    Boolean,        // bool

    // ─────────────────────────────────────────────
    // Variables & Declarations
    // ─────────────────────────────────────────────
    Variable,  // let
    Constant,  // const
    Alias,     // alias
    Declare,   // declare
    Entry,     // entry
    Schema,    // schema

    // ─────────────────────────────────────────────
    // Control Flow
    // ─────────────────────────────────────────────
    If,
    Else,
    When,
    Match,
    Case,
    Default,
    Switch,

    // ─────────────────────────────────────────────
    // Loops
    // ─────────────────────────────────────────────
    For,
    While,
    Loop,
    Until,
    Do,
    Break,
    Continue,

    // ─────────────────────────────────────────────
    // Functions & Returns
    // ─────────────────────────────────────────────
    Return,
    Yield,
    Exit,
    Cancel,

    // ─────────────────────────────────────────────
    // Async & Concurrency
    // ─────────────────────────────────────────────
    Async,
    Await,
		Spawn,
		Join,
    Coroutine,
		Atomic,
    Defer,
    Deferred,

    // ─────────────────────────────────────────────
    // Error Handling
    // ─────────────────────────────────────────────
    Try,
    Catch,
    Finally,
    Throw,

    // ─────────────────────────────────────────────
    // Modules & Imports
    // ─────────────────────────────────────────────
    Import,
    Export,
    From,
    As,
    Include,
    Use,

    // ─────────────────────────────────────────────
    // Types & Type System
    // ─────────────────────────────────────────────
    Type,
    Generic,   // <T>
    Is,        // is — type check
    Be,        // be
    Where,     // where — type constraint
    Extends,   // extends — inheritance
    Implements, // implements — interface impl
    In,        // in — membership
    Of,        // of — association
    Contains,  // contains — collection contains
    With,      // with — composition

    // ─────────────────────────────────────────────
    // Object-Oriented
    // ─────────────────────────────────────────────
    Class,
    Interface,
    Struct,
		Enum,
    Callable,
		Union,
		Intersection,
		Nullable,
    New,
    Super,     // super — base instance
    This,      // this — current instance
    SelfScope, // self — introspective scope
    Origin,    // origin — original object reference
    Root,      // root — object root
    Parent,    // parent — current parent
    Here,      // here — current location

    // ─────────────────────────────────────────────
    // Access Modifiers
    // ─────────────────────────────────────────────
    Public,
    Private,
    Protected,
    Internal,
    External,
    Global,
    Local,

    // ─────────────────────────────────────────────
    // OO Modifiers
    // ─────────────────────────────────────────────
    Static,
    Abstract,
    Virtual,
    Override,
    Final,
    Strict,

    // ─────────────────────────────────────────────
    // Declaration Flags
    // ─────────────────────────────────────────────
    Sorted,
    Unsorted,
    Unique,
    NotUnique,
		Mutable,
		Immutable,
		Lazy,
		Inline,
		Packed,

    // ─────────────────────────────────────────────
    // Reactive & Computed
    // ─────────────────────────────────────────────
    Reactive,
    Computed,
    Trigger,
    Untrigger,
    On,
    Set,
    Get,
    Unset,

    // ─────────────────────────────────────────────
    // Collection Operations
    // ─────────────────────────────────────────────
    Spread,
    Generate,
    Combine,
    Enumerate,
    Filter,
    Flatten,
    Repeat,
    Transform,
    Transpose,

    // ─────────────────────────────────────────────
    // Logic & Bitwise Operators
    // ─────────────────────────────────────────────
    And,
    Or,
    Not,
    Xor,
    Bitwise,
    Delta,

    // ─────────────────────────────────────────────
    // Capabilities & Permissions
    // ─────────────────────────────────────────────
    Provide,
    Capability,
    Condition,
    Only,
    May,

    // ─────────────────────────────────────────────
    // Metaprogramming
    // ─────────────────────────────────────────────
    Meta,
    Reflect,
    Attribute,
    Event,

    // ─────────────────────────────────────────────
    // Layout & Structure
    // ─────────────────────────────────────────────
    SectionMaker,
    Marker,
}
