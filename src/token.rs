#[derive(Debug, PartialEq, Clone, Default)]
pub enum Token {
    // Punctuation
    LParen,
    RParen,
    LCurly,
    RCurly,
    Semi,
    Comma,
    Colon,
    // Operators
    Plus,
    Minus,
    Assignment,
    Equality,
    Grt,
    Arrow,
    // Keywords
    Int,
    Let,
    Function,
    While,
    If,
    ElseIf,
    Else,
    Return,
    // Terminals
    Num(String),
    Id(String),
    // End
    #[default]
    Eof,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SyntaxToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Location {
    pub index: usize, // byte offset into source file
    pub line: usize,  // current line in the file; \n increments line
    pub col: usize,   // current position in the line
}

impl Location {
    pub fn new() -> Self {
        Self {
            index: 0,
            line: 1,
            col: 1,
        }
    }
}
