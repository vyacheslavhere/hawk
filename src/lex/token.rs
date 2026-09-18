/// Imports
use crate::common::span::Span;
use std::fmt::Debug;

/// Represents token kind
#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum TokenKind {
    If,        // `if` keyword
    Else,      // `else` keyword
    True,      // `true` keyword
    False,     // `false` keyword
    Nil,       // `nil` keyword
    Memoize,   // `memoize` keyword
    Echo,      // `echo` keyword
    Match,     // `match` keyword
    Use,       // `use` keyword
    Comma,     // ,
    Dot,       // .
    Lbrace,    // {
    Rbrace,    // }
    Lparen,    // (
    Rparen,    // )
    Lbracket,  // [
    Rbracket,  // ]
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    Amp,       // &
    Bang,      // !
    Bar,       // |
    Ge,        // >=
    Le,        // <=
    Gt,        // >
    Lt,        // <
    LtLt,      // <<
    RtRt,      // >>
    Colon,     // :
    Walrus,    // :=
    Pipe,      // |>
    DoubleEq,  // ==
    DoubleBar, // ||
    DoubleAmp, // &&
    BangEq,    // !=
    DoubleDot, // ..
    Number,    // any number
    String,    // "quoted text"
    Id,        // identifier
}

/// Defines a token, a structural unit lexer spits out
/// that contains it's kind, lexeme and source span
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub lexeme: String,
}

/// Token implementation
impl Token {
    /// Creates new token from specified span, kind and lexeme
    pub fn new(span: Span, kind: TokenKind, lexeme: String) -> Self {
        Self { span, kind, lexeme }
    }
}
